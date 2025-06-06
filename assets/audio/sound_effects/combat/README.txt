On combat,

When weapon swings, (REGARDLESS OF HIT OR MISS)
- Play one of wind_slash_1 through wind_slash_8 at random, and add some pitch variance to it

If weapon hits,
- If cooldown is at minimum (eg, 10ms),
  - Start playing weapon_hit_inf (the spam sound)

- Play weapon_hit_x, where x is the nearest number to the current cooldown. eg. if the current cooldown is 48 milliseconds, then use weapon_hit_0062

If weapon misses
- If weapon_hit_inf is currently playing,
  - Instantly stop it and play the next weapon_miss_x sound
- If weapon_hit_inf is not playing
  - Play weapon_miss_x, where x is the nearest number to the current cooldown. eg. if the current cooldown is 48 milliseconds, then use weapon_miss_0062

If the player stops swinging
- If weapon_hit_inf is currently playing
  - Play weapon_hit_inf_taper to fade out the inf sound effect
