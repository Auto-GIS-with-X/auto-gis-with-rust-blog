const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const eleventyNavigationPlugin = require("@11ty/eleventy-navigation");

module.exports = function (eleventyConfig) {
    eleventyConfig.addPlugin(syntaxHighlight);
    eleventyConfig.addPlugin(eleventyNavigationPlugin);

    return {
        templateFormats: [
            'html',
            'md',
            'njk',
            'css',
            'js',
            "svg"
        ],
        dataTemplateEngine: "njk",
        markdownTemplateEngine: "njk",
        htmlTemplateEngine: "njk",
        dir: {
            input: "src",
            output: "docs",
            layouts: "_layouts",
        },
        passthroughFileCopy: true,
        pathPrefix: "/auto-gis-with-rust-blog/"
    }
};