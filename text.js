const setResults = (id, value) => document.getElementById(id).innerText = value;
const clearResults = (id) => setResults(id, "");
const runTimedCode = (code) => {
    const startTime = performance.now();
    const results = code();
    const duration = performance.now() - startTime;
    return { results, duration };
}