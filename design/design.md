
To minimize resource usage, data should be queried only every second.
However the gui is to be drawn at 60hz or better.
Solution:
- if NowTime >= NextTime -> poll usage and set NextTime to NowTime + 1sec

For any data saved to json directly, put collected things into one "event_object" (cpu_log_DATE or smth else unique) and put those into the json file -> avoid truncation like eris currently does.
