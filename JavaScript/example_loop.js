n = 10;
for(var i = 0; i < n; i++) {
  // body
}
console.log(i);

for (let j = 0; j < n; j++) {
  // body
}
// console.log(j); // ReferenceError: j is not defined

// while (condition) {
//   // body
// }

let user = {
    name: "John",
    age: 30,
    isAdmin: true
};

for (const key in user) {
    console.log(key);
}

var conprehensive_test_num =10;
conprehensive_test_fun();
function conprehensive_test_fun()
{
    console.log(conprehensive_test_num);
    var conprehensive_test_num=20;
    console.log(conprehensive_test_num);
}