/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

// FIXME: add more sidebar chapters

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  // tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],

  // But you can create a sidebar manually
  
  mySidebar: [
    {
      type: 'category',
      label: 'Introduction',
      items: ['Intro/welcome', 'Intro/first', 'Intro/endpoints', 'Intro/data-access' ]
    },
    {
      type: 'category',
      label: 'In Depth',
      items: [ 'InDepth/cursors', 'InDepth/advanced-data', 'InDepth/pol', 'InDepth/secrets', 'InDepth/versions', 'InDepth/login', 'InDepth/routing', 'InDepth/chisel-cli' ],
    },
    {
       type: 'category',
       label: 'Examples',
       items: [ 'Examples/ex-gatsby', 'Examples/ex-nextjs' ] 
    },
    {
      type: 'category',
      label: 'Community',
      items: [ 'Community/feedback', 'Community/known-issues', 'Community/discord' ]
    },
  ],

};

module.exports = sidebars;
