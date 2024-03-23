# nyx: Nyx Ystem Xplorer
Nyx is a system monitor written in Rust. It provides real-time monitoring of CPU usage, memory consumption, disk I/O, network traffic, and other essential system metrics.

> [!NOTE]
> Perfomance of the gui may be lacking, but it is clearly made up by all the features nyx has!

## Roadmap

- Customizable logging feature.
	- Write important statisitcs to disc and query them.
	- 2 modes:
		1. Save it into an actual db for better quering
		2. Save it into several json files so any user can query the data how they want
- Customizable history depth of any chart
- Full controll over every process
- Support for various system architectures / gpu vendors
	- GPU monitoring of amd cards
	- Nvidia support maybe should I have one for testing.
	- ARM / AArch64 maybe should I have one for testing.
- Cli mode / version for even less ressource consumption
- A welcome message on startup with tipps!
- Backend v2 -> Now for Linux only for real! (Procfs is used in Linux / BSD but not by Windows or OSX)

## Features

- Real-time monitoring of system metrics
- Advanced chart colour technology - Use any colour you want!
- Extreme precision
- Timezone support
- Time and date display
- Settable settings
- Advanced settings
- A logo
- Success and error display
- Intuitive and classic design
- Choice between light and dark mode
- Currently only supports my systems specifically (Other systems could maybe, sometimes, work too!)
- A minimied view, perfect for monitoring the system on a second monitor using minimal space


## Why Use Nyx?

Nyx is a powerful and versatile system monitor that provides comprehensive insights into your system's health. It is lightweight and efficient, making it an ideal choice for both casual users and experienced system administrators.

With Nyx, you can:

- Keep an eye on your system's performance
- Identify potential bottlenecks
- Optimize resource usage

## How to use Nyx?

Nyx is easily installed, as the install wizard is the person present using the computer!

1. Compile from soucre as executables are not provided, get better scrub
	- If any are provided they will probably not work anyways!
2. Put the logo.jpeg into you pictures folder.
	- No logo no Nyx! It took me several minutes to generate and cost me almost a halfcent in electricity, so I will force it on everyone!
3. Pray it works.
4. ???
5. Use Nyx like the aplha human you clearly are!

## Nyx: A Fitting Name for a System Monitor

Nyx, the ancient Greek goddess of night, is a fitting name for a system monitor. This powerful deity embodies the duality of night, both its darkness and its tranquility. Just as Nyx is both feared and respected, a system monitor can be both a source of anxiety and a tool for understanding and optimizing your system's health.

Night's Duality

Nyx is a primordial goddess, born from Chaos, the void from which all existence emerged. She is the mother of many powerful deities, including Aether, the personification of light, and Hemera, the personification of day. Yet, Nyx herself is associated with darkness and the shadows.

This duality is reflected in the role of a system monitor. On the one hand, it can expose the darker aspects of your system's health, such as high CPU usage, memory leaks, or potential security vulnerabilities. This can be unsettling, as it highlights the potential problems that could be lurking within your system.

Nyx is not merely a force of darkness; she is also associated with wisdom and foresight. She is said to be the mother of Moros, the personification of fate, and Nemesis, the goddess of retribution. This connection to fate and retribution suggests that Nyx understands the consequences of our actions, both good and bad.

In the same way, a system monitor can help you understand the reasons of your system's behavior.

### Nyx, a recurxive acronym?
Yes, Nyx is, as all good names, a recursice acronym:

- Nyx
- Ystem
- Xplorer

As Nyx truly is, one of the sYstem eXplorer's ever made!

## Acknowledgments
Thanks to the open-source community for providing invaluable tools and libraries.
Used in this project:
- [chrono](https://crates.io/crates/chrono)
- [chrono-tz](https://crates.io/crates/chrono-tz)
- [eframe](https://crates.io/crates/eframe)
- [sysinfo](https://crates.io/crates/sysinfo)
- [dirs](https://crates.io/crates/dirs)
- [mexprp](https://crates.io/crates/mexprp)
- [procfs](https://crates.io/crates/procfs)
- [rand](https://crates.io/crates/rand)
- [image](https://crates.io/crates/image)
- [json](https://crates.io/crates/json)
