# Day 13: Transparent Origami

In today's solution, I tried to be liberal in my use of structs, and I'd say it turned out well. The code communicated exactly what it was trying to say, giving meaning to abstract types. I need to be better about using my own data types in other contexts. Nothing wrong with defining a type to convey meaning; usually it's pretty cheap and pays off in the long run.

Today's solution was surprisingly easy compared to what I expected. Having a unicode-friendly language eased the display of the final set of letters:

```
███   ██  █  █ ███  █  █ █    █  █ █
█  █ █  █ █  █ █  █ █ █  █    █ █  █
█  █ █    ████ █  █ ██   █    ██   █
███  █ ██ █  █ ███  █ █  █    █ █  █
█    █  █ █  █ █ █  █ █  █    █ █  █
█     ███ █  █ █  █ █  █ ████ █  █ ████
```

All you need is a unicode-friendly terminal...