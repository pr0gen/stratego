const rust = import('./pkg');

rust
  .then(m => m.fibo(6))
  .catch(console.error);

function hello_world() {
  return "bonjour from javascript";
}
