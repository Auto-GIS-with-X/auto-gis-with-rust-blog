const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");

module.exports = function (eleventyConfig) {
    eleventyConfig.addPlugin(syntaxHighlight);

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