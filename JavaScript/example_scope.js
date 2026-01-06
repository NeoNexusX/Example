// function scopeExample() {
//     var functionVar = "I am a function variable";
//     console.log(functionVar); // Accessible here
// }
// scopeExample();
// console.log(functionVar); // Uncaught ReferenceError: functionVar is not defined

function scopeExample() {
    globalVar = "I am a global variable";
    console.log(globalVar); // Accessible here
}
scopeExample();
console.log(globalVar); // Accessible here

function blockScopeExample() {
    if (true) {
        let blockVar = "I am a block variable";
        // let blockVar = "I am a block variable redeclared"; // This will cause an error
        console.log(blockVar); // Accessible here
    }
    // console.log(blockVar); // Uncaught ReferenceError: blockVar is not defined
}
blockScopeExample();

function constExample() {
    const constantVar = "I am a constant variable";
    console.log(constantVar); // Accessible here
    //constantVar = "Trying to change constant variable"; // This will cause an error
}
constExample();

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

//Example5
console.log("Example5:");
var num_chain=10
function fun_chain_outside()//外部函数
{
    var num_chain =20
    function fun_chain_inside()//内部函数
    {
        console.log(num_chain);
    }
    fun_chain_inside();
}
fun_chain_outside();