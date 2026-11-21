/**
 * webpack loader: repairs the prebuilt bundle of @sctg/rapidoc 9.3.8-b.
 *
 * Why: that bundle (dist/rapidoc-min.js) contains Prism's `jsdoc` language, which is defined with
 * `Prism.languages.extend("javadoclike", ...)`, but not the `javadoclike` language it extends. Evaluating the
 * bundle therefore throws "Cannot set properties of undefined (setting 'parameter')" and /api/doc stays empty.
 * The bundle carries its own copy of Prism (it replaces the global one), so the fix cannot be made from our own
 * code: the missing language has to be defined inside the bundle, just before `jsdoc` uses it.
 *
 * How: the real `prism-javadoclike.js` of the installed `prismjs` is inserted in front of the `jsdoc`
 * definition, with `Prism` bound to the bundle's own instance (`e`, the argument of that module).
 *
 * Remove this loader (and its rule in webpack.config.cjs) once @sctg/rapidoc ships a fixed bundle: when the
 * anchor below is no longer found the loader does nothing.
 */
const fs = require("fs");

const ANCHOR = 'e.languages.jsdoc=e.languages.extend("javadoclike"';

module.exports = function patchRapidoc(source) {
  const text = source.toString();
  // already defined by the bundle (a fixed release) or no jsdoc at all (older releases): nothing to repair
  if (!text.includes(ANCHOR) || /languages\.javadoclike=\{/.test(text)) {
    return source;
  }
  const javadoclike = fs.readFileSync(
    require.resolve("prismjs/components/prism-javadoclike.js"),
    "utf8"
  );
  return text.replace(ANCHOR, `(function(Prism){${javadoclike}\n})(e);${ANCHOR}`);
};
