# DEVELOPMENT HAS PAUSED
 I've been getting tired of trying to fit an ECS around with Rust's borrowing rules, so I'm taking a break to learn more about Rust and make a non ECS engine.<br>
 You are still free to use this engine, and fork it, and look into it :)

# About
 This is Berry.rs! My engine for Rust, Macroquad and HECS!

# How
 Clone this repository and use it!
 
 Here's HECS' repository so you can learn how to spawn and use entities, components and systems: https://github.com/ralith/hecs
 Add your systems into Master.update() and Master.render() (most of them will need a reference to the Master and World)
 
 And, here's Macroquad's repository so you can learn rendering, keyboard input and sound: https://github.com/not-fl3/macroquad
 Put all of your Texture, Sound, and Font loading into Resources.load()
 (TIP: Make all of your Sounds, Option<Sound> so that in Resources::new() you can say None)

# Copyright
 You are free to use this, as long as you give credit :D
