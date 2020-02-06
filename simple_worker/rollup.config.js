export default [
    {
        input: "src/worker.js",
        output: {
            file: "pkg/worker.js",
            format: "iife",
            resolveImportMeta: null
        },
        plugins: [
            myPlugin(),
        ]
    }
];

function myPlugin() {
    return {
        name: "hook",
        resolveImportMeta(property, { moduleId }) {
            if (property === "url") {
                return "_";
            }

            return null;
        }
    };
}
