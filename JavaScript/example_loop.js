// Example1: var in for-loop leaks to outer scope
console.log("Example1:");

const n = 10;

for (var i = 0; i < n; i++) {
  // body
}
console.log(i);

// Example2: let in for-loop is block-scoped
console.log("Example2:");

for (let j = 0; j < n; j++) {
  // body
}
// console.log(j); // ReferenceError: j is not defined

while (false) {
  // body
}

let user = {
    name: "John",
    age: 30,
    isAdmin: true
};

// Example3: for...in iterates object keys
console.log("Example3:");

for (const key in user) {
    console.log(key);
}