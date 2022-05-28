# EVEBashSignal
Find nearby Bashes in EVE by searching recent killmails for large amounts of expensive ships lost. Send a Discord notification when this occurs with information regarding where the fight is going down.

## Process
- Use zkillboard apis to get killmails. This will be live data, and will not be saved.
    - Start with [RedisQ](https://github.com/zKillboard/RedisQ). Maybe websocket later if project is expanded
- take killmails and filter them by desired ships/corps/people, etc. Maybe this is done as a tui, maybe as args
- a threshold is to be set for a certain filter. When this threshold is breached, a discord notification is sent.