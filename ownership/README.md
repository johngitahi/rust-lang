# Ownership & Borrowing
This has been a very interesting concept to learn esp since I have a little basic understanding of how C pointers work. I can say it has reinforced my understanding of them. Here are a list of very important things I need to underscore well before proceeding to more concepts or even building some project with Rust:

**Basic scenario: When I pass a variable to a function as a parameter**

1. The variable will go out of its scope and into the functions scope. After the function is done wit it, the variable dies out(basically it won't be used again, so calling it from its previous scope will result in an error)

2. To make the variable not go out of scope into the function's scope, I need to use references(&) which will lend the value of the variable to the function, but not give its ownership, hence *borrowing*

3. The concept of heaps and stacks is crucial to understanding how this whole thing works, so in case I come back here and i dont understand them, I should just go back and relearn them.(Guess that's why CS is very important) Below is how I understood shit.

4. Heaps store data unorderly(that's how i understood it), data is stored on an as-needed-basis. It basically chooses a chunk of memory to store your data based on its size. Stacks work in a manner opposite to heaps; they store data on a first come first stored basis. So the latest comer will be at the top, therefore, when anyone needs to  be execeuted(death) first, they pick the latecomer, the latest data on the stack. While for heaps, they'd go through a democratic process of voting out who would be executed first. That's a simple way of understanding it.

- From the analogy above, it is clear that getting data out of heaps is slower as compared to stacks.

## Now why understand heaps and stacks first?

Well, because data in Rust is stored in both manners:
- The actual value is stored in a heap
- The stack will hold the pointer for the position of the actual values(which are in the heap)

Example: A integer data structure: `String` with a value `Janja`

The actual value 32 is stored in a heap, this way:
    - The stack holds the metadata(variable_name, size, and most importantly, a pointer to where the real value is)
    - The heap holds the real value
Visually: 

    Stack               Heap
    -----               ----
    ptr         ----> | "Janja" |
    (name, size,etc)   --------




## Important things to recall
- References, like variables are immutable by default
- You can have more than one immutable reference but only one mutable reference in the same scope, for reasons well understood -> (To prevent a scenario where one mut ref changes data so the immut refs have to decide which value to pick; the previous one or the one edited by the mut ref{ referred to as **data races**})
- You cannot allocate an immutable reference and a mutable reference in the same scope
- A walkaround for this is to use a different scope or let the first immutable ref die by using it first then creating the immutable reference. Example

``` rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

```

From The Rust Book ----↓
> Even though borrowing errors may be frustrating at times, remember that it’s the Rust compiler     pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is. Then you don’t have to track down why your data isn’t what you thought it was.

That's what I learnt today!
