Ref A (Polling rate):
A1: To minimize resource usage, data should be queried only every second.
A2: However the gui is to be drawn at 60hz or better.
		Solution:
		- if NowTime >= NextTime -> poll usage and set NextTime to NowTime + 1sec

Ref B (saving directly):
B1: For any data saved to json directly, put collected things into one "event_object" (cpu_log_DATE or smth else unique) and put those into the json file -> avoid truncation like eris currently does.

Ref C (Core count):
C1: Manycore systems or supercomputers are out of scope for this project.
		CPU:
			Consumer grade cpu's will not reach a core count of more than 255 (u8) for the foreseeable future.
				- As of Jan 2024 the highest core count is 96. Threadripper 7955WX (10.000$)

Ref D (Timestamps):
D1: Saving of timestamps
		Nyx queries data once per second (This could be done in a seperate thread)
			Timestamps for systemdata only need seconds to be unique. Especially since other computation time could impact the polling rate.

## UI

Ref UIa (Unix):
UIa1: Keep ui unix philosophy -> as easy as possible, best case: user doesn't notice ui, just interacts with it.
UIa2: Landing page should have all one needs. Subpages should only be needed for deeply detailed info.
		- All cpu cores + Avg sys load
		- Ram fill status
		- GPU usage
		- Network usage
		- Disc usage
UIa3: A main menu button row on top is adviceable for unix -> If a user doesn't know where a feature is, the first place one does is look up and click through the menus.
	- Shortcuts (like clicking on the cpu charts to go to the cpu menu) can be explained in a seperate "help" menu
		- Where noone would ever look at them
		- However, the "help" page pairs well with an "about" page for Licenses and acknowledgements.
	- Or shortcuts can be explained at the top of each menu when you reach it, the first time by using the main menu.
	- Implementing both would be the approch used for Ananke, but instead of combining "about" and "help" they would be seperate sub-menus along with a "quit" button, just in case. I don't want someone to have a vim experience.
UIa4: The left over space right of the main menu, could be filled with the "Nyx version X.Y.Z by Xqahre, (Nyx)[github]" text

Ref UIb (Settings):
UIb1: Do I need a "settings" place?
		- eframe handles light and dark mode automatically from system theme, it could be toggleable though.
			- Set custom colours for graphs and stuff would neccessitate a settings page.
		- eris settings?
			- could be split up by system component as its own subservice, with readout and visualiation
			- Or be its own subpage with overview in landing page

