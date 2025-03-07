import { convert } from "html-to-text";
import path from "path";
import fs from "fs/promises";
import { dump } from "js-yaml";
import dedent from "dedent";
import clipboardy from "clipboardy";
import { logger } from "../../logger";
import { EmailTransporter, EmailTransporterSendMailOptions } from "./types";

interface PlainEmailDump {
  date: string;
  from: string;
  to: string;
  subject: string;
  text?: string;
  html?: string;
}

type DerivedPayload =
  | { payloadType: "unknown" }
  | {
      payloadType: "loginVerification";
      verificationCode: string;
      magicLink: string;
    }
  | {
      payloadType: "signupVerification";
      verificationCode: string;
      magicLink: string;
    }
  | {
      payloadType: "orgInvitation";
      orgName: string;
      invitationLink: string;
      invitationLinkOrgEntityId: string;
      invitationLinkToken: string;
    };

type DerivedPayloadType = DerivedPayload["payloadType"];

interface EmailDump extends PlainEmailDump {
  derivedPayload: DerivedPayload;
}

const yamlFileHeader = dedent`
  ## This file has been autogenerated by DummyEmailTransporter.
  ## The contents are updated every time the API server sends an email.
  ## Most recent emails are shown at the top, like in an inbox.
  ##
  ## Note that the file is emptied when the API server is restarted.
`;

/**
 * The transporter does not have direct access to the semantic meaning of the emails.
 * However, we can extract bits from it by parsing the dump, similar to what humans do
 * in their inboxes. Derived data is used when communicating codes or links in dev mode.
 * Tests can access the derived payload too and thus avoid html parsing.
 */
const derivePayload = (plainEmailDump: PlainEmailDump): DerivedPayload => {
  try {
    const { subject, html } = plainEmailDump;
    if (subject === "Your HASH verification code") {
      return {
        payloadType: "loginVerification",
        verificationCode: html!.match(/<code>(.*)<\/code>/)![1]!,
        magicLink: html!.match(/href="(.*)"/)![1]!,
      };
    }

    if (subject === "Please verify your HASH email address") {
      return {
        payloadType: "signupVerification",
        verificationCode: html!.match(/<code>(.*)<\/code>/)![1]!,
        magicLink: html!.match(/href="(.*)"/)![1]!,
      };
    }

    if (subject === "You've been invited to join an organization at HASH") {
      const invitationLink = html!.match(/href="(.*)"/)![1]!;

      return {
        payloadType: "orgInvitation",
        orgName: html!.match(/<strong>(.*)<\/strong>/)![1]!,
        invitationLink,
        invitationLinkOrgEntityId: invitationLink.match(
          /orgEntityId=([\w-]+)&/,
        )![1]!,
        invitationLinkToken: invitationLink.match(
          /invitationEmailToken=(\w+)&/,
        )![1]!,
      };
    }
  } catch (error) {
    throw new Error(
      `Unable to parse plainEmailDump: ${JSON.stringify(
        plainEmailDump,
      )}\n\n${error}`,
    );
  }

  return { payloadType: "unknown" };
};

export interface DummyEmailTransporterConfig {
  copyCodesOrLinksToClipboard?: boolean;
  displayCodesOrLinksInStdout?: boolean;
  filePath?: string;
  from?: string;
}

const defaultFrom = "dummy-email-transporter@hash.test";

export class DummyEmailTransporter implements EmailTransporter {
  /**
   * Emails are ordered from newest [0] to oldest [length - 1]. This makes
   * it simpler to read the yaml without having to reorder items in it.
   */
  private emailDumps: EmailDump[] = [];

  constructor(private config: DummyEmailTransporterConfig = {}) {
    void this.writeEmailsToFile();
  }

  async sendMail({ to, subject, html }: EmailTransporterSendMailOptions) {
    const plainEmailDump: PlainEmailDump = {
      date: new Date().toString(),
      from: this.config.from ?? defaultFrom,
      to,
      subject,
      text: convert(html),
      html,
    };

    const emailDump = {
      ...plainEmailDump,
      derivedPayload: derivePayload(plainEmailDump),
    };

    await this.communicateCodesOrLinks(emailDump);

    this.emailDumps.unshift(emailDump);
    await this.writeEmailsToFile();
  }

  getMostRecentEmail<T extends DerivedPayloadType>(options: {
    assertDerivedPayloadType: T;
  }): EmailDump & { derivedPayload: { payloadType: T } };

  getMostRecentEmail(): EmailDump | undefined;

  getMostRecentEmail(options?: {
    assertDerivedPayloadType: DerivedPayloadType;
  }): EmailDump | undefined {
    const result = this.emailDumps[0];

    if (!options?.assertDerivedPayloadType) {
      return result;
    }

    if (!result) {
      throw new Error("No emails have been sent yet");
    }

    if (
      result.derivedPayload.payloadType !== options.assertDerivedPayloadType
    ) {
      throw new Error(
        `Expected most recent email to have derived payload type "${
          options.assertDerivedPayloadType
        }", got ${JSON.stringify(result)} `,
      );
    }

    return result;
  }

  private async communicateCodesOrLinks(emailDump: EmailDump): Promise<void> {
    const { copyCodesOrLinksToClipboard, displayCodesOrLinksInStdout } =
      this.config;

    const rowsToDisplay: string[] = [`New email to ${emailDump.to}!`, ""];

    switch (emailDump.derivedPayload.payloadType) {
      case "loginVerification":
        rowsToDisplay.push(
          "Login link:",
          emailDump.derivedPayload.magicLink,
          "",
          "Verification code:",
          emailDump.derivedPayload.verificationCode,
        );
        break;
      case "signupVerification":
        rowsToDisplay.push(
          "Signup link:",
          emailDump.derivedPayload.magicLink,
          "",
          "Verification code:",
          emailDump.derivedPayload.verificationCode,
        );
        break;
      case "orgInvitation":
        rowsToDisplay.push(
          "Org invitation link:",
          emailDump.derivedPayload.invitationLink,
        );
        break;
    }

    if (!rowsToDisplay.length) {
      return;
    }

    if (copyCodesOrLinksToClipboard) {
      try {
        await clipboardy.write(rowsToDisplay[rowsToDisplay.length - 1]);
      } catch {
        // Prevent hard crash on Ubuntu without xsel installed (e.g. in CI)
        rowsToDisplay.push("(could not copy to clipboard)");
      }
      rowsToDisplay.push("(copied to clipboard)");
    }

    if (!displayCodesOrLinksInStdout) {
      return;
    }

    const maxAllowedRowWidth = process.stdout.columns ?? 40;
    let maxRowWidth = 10;
    rowsToDisplay.forEach((rowToDisplay) => {
      const rowWidth = rowToDisplay.length;
      if (rowWidth > maxRowWidth && rowWidth <= maxAllowedRowWidth) {
        maxRowWidth = rowWidth;
      }
    });

    rowsToDisplay.unshift("=".repeat(maxRowWidth));
    rowsToDisplay.unshift("");
    rowsToDisplay.push("=".repeat(maxRowWidth));
    rowsToDisplay.push("");

    rowsToDisplay.forEach((rowToDisplay) => {
      process.stdout.write(`${rowToDisplay}\n`);
    });
  }

  private async writeEmailsToFile(): Promise<void> {
    if (!this.config.filePath) {
      return;
    }

    try {
      await fs.mkdir(path.dirname(this.config.filePath), {
        recursive: true,
      });

      const yamlFileContents = this.emailDumps.length
        ? `${yamlFileHeader}\n\n---\n${this.emailDumps
            .map((email) => dump(email, { lineWidth: -1 }))
            .join("\n---\n")}`
        : yamlFileHeader;

      await fs.writeFile(this.config.filePath, yamlFileContents, "utf-8");
    } catch (error) {
      logger.error(error);
    }
  }
}
