# Kour Spammer

Kour Spammer is a simple program that allows you to spam the global chat in kour.io,
you can use any username or clan you would like.

![screenshot](assets/example.png)

## Usage
```
Usage: kour_spam [message]
```

## Customizing
You can customize what message will be spammed in the chat by modifing the `MESSAGE` constant inside `src/main.rs`.

you can send any message with attributes such as aliases for colors,
hex representations of rgba colors, and verified checkmark with `<sprite=0>`.

## How does it work
The reason why this works is because kour.io does not have any sort of authentication for sending messages,
this means anyone from anywhere without even having a account can send a post request to `https://kour.io/api/message`,
an example json payload may look like this.

```json
{ "message": "<sprite=0><color=white><color=#7300FFFF>[DANK] </color>kashy</color>: <color=#e8e8e8> hello there!" }
```


