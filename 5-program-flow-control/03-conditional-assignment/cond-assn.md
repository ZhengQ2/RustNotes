> Consider the following code:
> <pre>
> fn main () {
>    let make_x_odd = true;
>    <i>let x;
>
>    if make_x_odd {
>        x = 1;
>    } else {
>        <b>x = 2;</b>
>    }</i>
>
>    println!("x is {}", x);
> }
> </pre>
> Since make_x_odd is always true, so we can comment out the line **in bold** right?

No, we can't. The compiler will complain:  `error[E0381]: use of possibly uninitialized variable: 'x'`    

Why? Because the compiler doesn't know that x will be initialized in the if-else block. It's possible that the if-else block is never executed, and x is never initialized. This is called a "use after move" error. The compiler is trying to protect us from using a variable that may not have been initialized.

However, the *italic part* of the code is common enough, that we can replace it with a single line: `let x = if make_x_odd {1} else {2};`

This is equivalent to C's: `int x = make_x_odd ? 1 : 2;`

However,
> Note: in this case, two variables must be the same data type, i.e., both integer, or both float-point.
