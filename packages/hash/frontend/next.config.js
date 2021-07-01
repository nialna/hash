module.exports = {
  async redirects() {
    return [
      {
        source: "/",
        destination: "/2/page1",
        permanent: false,
      },
    ];
  },
  sassOptions: {
    prependData: `
      $grey-bg: rgba(241, 243, 246, 0.3);
      $grey-border: #e5e6e7;
      $black-almost: #1b1d24;
    
      $bright-purple: rgb(95, 71, 255);
      $bright-pink: #ff008b;
      $bright-blue: #2482ff;  
    `,
  },
};
