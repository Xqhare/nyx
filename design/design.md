
To minimize resource usage, data should be queried only every second.
However the gui is to be drawn at 60hz or better.
Solution:
- if NowTime >= NextTime -> poll usage and set NextTime to NowTime + 1sec

For any data saved to json directly, put collected things into one "event_object" (cpu_log_DATE or smth else unique) and put those into the json file -> avoid truncation like eris currently does.
Interception of SIGTERM, should help with that too, even if it hasn't worked with eris just yet.

If user wants to exit, I could try and send myself a SIGTERM signal and trigger a graceful shutdown.
	If this doesn't work I can add a condition inside the thread blocking loop that breaks it, but this would be sub optimal.

## UI

Keep ui unix philosophy -> as easy as possible, best case: user doesn't notice ui, just interacts with it.

Landing page should have all one needs. Subpages should only be needed for deeply detailed info.
	- All cpu cores + Avg sys load
	- Ram fill status
	- GPU usage
	- Network usage
	- Disc usage

Do I need a "settings" place?
	- eframe handles light and dark mode automatically from system theme, it could be toggleable though.
	- eris settings?
		- could be split up by system component as its own subservice, with readout and visualiation
		- Or be its own subpage with overview in landing page

