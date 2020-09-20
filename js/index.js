import("../pkg/index.js")
  .then(go)
  .catch(console.error);

function go(wasm) {
  const ids = [...Array(1000).keys()];
  let input;

  preamble();

  input = ids.map(id => ({ value: "cluster-" + id }));
  bench("strings_json", 100, () => {
    wasm.strings_json(input);
  });
  bench("strings_serde_wb", 100, () => {
    wasm.strings_serde_wb(input);
  });
  bench("strings_serde_wb_complete", 100, () => {
    wasm.strings_serde_wb_complete(input);
  });

  input = ids.map(id => ({ value: id }));
  bench("numbers_json", 100, () => {
    wasm.numbers_json(input);
  });
  bench("numbers_serde_wb", 100, () => {
    wasm.numbers_serde_wb(input);
  });
  bench("numbers_serde_wb_complete", 100, () => {
    wasm.numbers_serde_wb_complete(input);
  });

  trailer();
  document.body.innerHTML = benchmarkTable;
}

let benchmarkTable = "";

function bench(name, iterations, fn) {
  let time1 = performance.now();

  // Perform the benchmark.
  for (let i = 0; i < iterations; i++) {
    fn()
  }

  // Get end time and compute elapsed milliseconds.
  let time2 = performance.now();

  const round = n => Number((n).toFixed(6));
  let elapsed = time2 - time1;
  let per_iter = elapsed / iterations;

  benchmarkTable += `<tr>
    <td>${name}</td>
    <td>${round(elapsed)} ms [${round(per_iter)} ms / iter]</td>
  </tr>\n`;
}

function preamble() {
  benchmarkTable += `
  <table>
    <thead>
      <tr>
        <th>Benchmark</th>
        <th>Time</th>
      </tr>
    </thead>
    <tbody>
`;
}

  function trailer() {
  benchmarkTable += `
    </tbody>
  </table>
`;
}
