import { Express } from "express";
import { DBAdapter } from "../db";
import { setupPassport } from "./passport";
import { setupSession, SessionConfig } from "./session";
import { Config as PgConfig } from "../db/postgres";

const setupAuth = (
  app: Express,
  sessConfig: SessionConfig,
  pgConfig: PgConfig,
  db: DBAdapter,
) => {
  // setup session related middleware
  setupSession(app, sessConfig, pgConfig);
  // setup passport related middleware and routes
  setupPassport(app, db);
};

export default setupAuth;
