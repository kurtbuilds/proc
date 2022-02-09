<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/procs/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/procs.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/procs/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/procs.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/procs/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/procs/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/procs">
    <img src="https://img.shields.io/crates/d/procs?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/procs">
    <img src="https://img.shields.io/crates/v/procs?style=flat-square" alt="Crates.io" />
</a>

</p>

# proc

`proc` makes it easy to find and manage system processes. Right now, the main usage is finding processes by the ports 
it is listening on, but more features are planned.

# Usage

    # Find the process listening on port 5000
    proc -p 5000 

    # Find the process listening on port 5000 and print the process name
    proc -ap 5000

    # Find the process listening on port 5000 and kill it  
    proc -p 5000 -- kill

# Installation

    cargo install proc-find

# Roadmap

# Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request
