# nyx: Nyx Ystem Xplorer
Nyx is a system monitor written in Rust. It provides real-time monitoring of CPU usage, memory consumption, disk I/O, network traffic, and other essential system metrics.

> [!NOTE]
> Perfomance of the gui may be lacking, but it is clearly made up by all the features nyx has!

## Features / Roadmap

- Real-time monitoring of system metrics
- Customizable logging feature.
	- Write important statisitcs to disc and query them.
- Support for various system architectures / gpu vendors
	- Currently AMD on Linux x86-64 only.
	- Nvidia support maybe should I have one for testing.
	- ARM / AArch64 maybe should I have one for testing.
- Cli mode / version for even less ressource consumption

## Why Use Nyx?

Nyx is a powerful and versatile system monitor that provides comprehensive insights into your system's health. It is lightweight and efficient, making it an ideal choice for both casual users and experienced system administrators.

With Nyx, you can:

- Keep an eye on your system's performance
- Identify potential bottlenecks
- Optimize resource usage

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
- [libdrm_amdgpu_sys](https://crates.io/crates/libdrm_amdgpu_sys)
- [dirs](https://crates.io/crates/dirs)
- [mexprp](https://crates.io/crates/mexprp)
- [procfs](https://crates.io/crates/procfs)
- [rand](https://crates.io/crates/rand)
