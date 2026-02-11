# nyx: Nyx Ystem Xplorer
Nyx is a system monitor written in Rust. It provides real-time monitoring of CPU usage, memory consumption, disk I/O, network traffic, and other essential system metrics.

Nyx truly is, one of the sYstem eXplorer's ever made!

## Naming

Nyx was the ancient Greek goddess of the night, one of the primordial gods (protogenoi) who emerged as the dawn of creation.

## Rework

This project was abandoned after running into massive performance issues and eventually gave up on it.
Now, with the addition of my home-lab, I have the need for a custom system monitor once again.
As with all my other projects its mainly educational as I restrict myself to only self-written code and the standard library.

To improve the horrible performance of the previous iteration, the project design has been completely reworked:

- Split the project into a back- and frontend.
- The backend will be almost entirely separated from the frontend.
- This will allow the frontend to focus solely on displaying the current system state (as determined by the backend).
	- 60 fps - I mean it's a TUI, it's gonna need all the frames!
- The backend will focus on collecting data from the system, processing and structuring it and sending it to the frontend.

Instead of actually using (and learning) actual `async` programming, I will use my IPC framework [hermes](https://github.com/xqhare/hermes) to create a multi-process system. This way, the biggest bottleneck is disk I/O, which should still allow for second to maybe even half second update times.

Right now I believe I will need most of my already written eco-sytem:

- [hermes](https://github.com/xqhare/hermes) for IPC
- [athena](https://github.com/xqhare/athena) for working with `XffValues`
- [horae](https://github.com/xqhare/horae) for Time and Date
- [talos](https://github.com/xqhare/talos) for TUI

- proc/meminfo OR `free -h`
	- first 3 lines:
		- MemTotal (kb)
		- MemFree (kb)
		- MemAvailable (kb)
- proc/mounts OR `df -h`
	- first entry (whitespace separated):
		- /dev/DEVICE (except loop devices)

(Having to use `sudo` is solved by adding the executing user to the docker group - as I have done on the home-lab)
- run `sudo docker ps -a`
	- parse output
	- display table:
		- Name
		- Status
		- uptime
- check `shamash/shamash-logs` for `network_`, `isp_` and `local_outage_ongoing` files for display of current status
	- no file == online!
- run `ps -eo user,pid,cmd,%mem,%cpu --sort=-%cpu | head -n 15`
	- parse output & Display
	- cpu and mem usage needs to be divided by number of cores
- run `uptime`
	- parse output & Display
		- Uptime
		- Load average
			- 1, 5, 15 minutes. Numbers need to be divided by number of cores


### Things to keep from previous iteration

- Only supports my systems specifically (Other systems could maybe, sometimes, work too!)
- Time and date display
	- Timezone support (Maybe?? - this would really benefit from again having a config file - And settings menu!)
- Success and error display
- Intuitive and classic design
- Real-time monitoring of system metrics

### Things to add

- FPS counter & Backend Update time (because funny)

## OLD README - use for inspiration

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

- Advanced chart colour technology - Use any colour you want!
- Extreme precision
- Settable settings
- Advanced settings
- A logo
- Choice between light and dark mode
- A minimied view, perfect for monitoring the system on a second monitor using minimal space

