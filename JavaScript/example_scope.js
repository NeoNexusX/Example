// function scopeExample() {
//     var functionVar = "I am a function variable";
//     console.log(functionVar); // Accessible here
// }
// scopeExample();
// console.log(functionVar); // Uncaught ReferenceError: functionVar is not defined

// Example1: Implicit global variable (function writes to global scope)
console.log("Example1:");

function scopeExample() {
    // Intentionally omit var/let/const to demonstrate an implicit global.
    globalVar = "I am a global variable";
    console.log(globalVar); // Accessible here
}
scopeExample();
console.log(globalVar); // Accessible here

// Example2: Block scope with let
console.log("Example2:");

function blockScopeExample() {
    if (true) {
        let blockVar = "I am a block variable";
        // let blockVar = "I am a block variable redeclared"; // This will cause an error
        console.log(blockVar); // Accessible here
    }
    // console.log(blockVar); // Uncaught ReferenceError: blockVar is not defined
}
blockScopeExample();

// Example3: const (cannot be reassigned)
console.log("Example3:");

function constExample() {
    const constantVar = "I am a constant variable";
    console.log(constantVar); // Accessible here
    //constantVar = "Trying to change constant variable"; // This will cause an error
}
constExample();

// Example4: Hoisting with var (declaration hoisted, assignment not)
console.log("Example4:");

function hoistingExample() {
    console.log(a);
    var a = 10;
    console.log(a);
}
hoistingExample();

function hoistingLetExample() {
    // console.log(b); // Uncaught ReferenceError: Cannot access 'b' before initialization
    let b = 20;
    console.log(b);
}
hoistingLetExample();

// Example5: Scope chain (inner function reads outer variable)
console.log("Example5:");
var num_chain = 10;

function fun_chain_outside() // 外部函数
{
    var num_chain = 20;

    function fun_chain_inside() // 内部函数
    {
        console.log(num_chain);
    }
    fun_chain_inside();
}
fun_chain_outside();


// Example6: Function declaration hoisting (can call before definition)
console.log("Example6:");

function_fun(); // 可以执行
function function_fun() {
    console.log("test");
}

// //Example7 fun_expression hoisting (cannot call before definition)
// console.log("Example7:");

// fun_expression();

// var fun_expression = function()
// {
//     console.log("pre_var_fun_test");
// }

// Example8: Hoisting 
console.log("Example8:");

var outside = 10;

conprehensive_test_fun();

function conprehensive_test_fun() {
    console.log(outside);
    var outside = 20;
}
