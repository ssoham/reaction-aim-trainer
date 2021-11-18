# Reaction & Aim Trainer

- Project for CS128H [Group 8]

Developed by Soham S. (soham3) & Rohit K. (rkundu3)

## Introduction
The final project will be two-part with one final application. The first part of the program will test one's reaction speed given by different color changes, and the second is to improve one's accuracy when clicking targets. The idea came when we were playing around with benchmark tests and games, as well as finding the [quicksilver](https://github.com/ryanisaacg/quicksilver) framework for developing small games.

## System Overview
- Final project would be GUI-based with no CLI apart from possibly another way to run the program.
- The mini-project on reaction speed will utilize the ability of changing background colors from `quicksilver` as well as reading system time values through either built-in or external rust `crate`s. 
- The mini-project regarding clicking accuracy will also rely on time stamps, in addition to accessing pixels and mouse movement. 

## Possible Challenges
There are quite a few challenges that could pop up, such as being unable to properly read pixel values. Another issue we could run into would be making an entire GUI based application (and possibly converting this to an executable file), as we haven't done any visual FX work in Rust. A third challenge could arise from the `quicksilver` framework; though it did look promising, we hadn't looked much into it so the features could be limiting in which case other alternatives for GUI applications would need to be found.

### Required Packages
```
apt install libudev-dev
apt install pkg-config
```
