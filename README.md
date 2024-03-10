# Project Joshu - joshu-core

I'll explain what this whole thing is later on. Also, not everything here has been implemented yet.

## API

You send messages to joshu-core over `stdin`.
When the command specified in the message has finished executing, joshu-core sends its response via `stdout`.

Every message and response is in json.

### Basic message format

```json
{
  "id": 0,
  "avatar_emotion": "normal",
  "textbox_text": "Hello world!",
  "canvas_mode": {}
}
```

Every field is optional.

- `id` is pretty much irrelevant. If it's given in the message, it'll be there in the response as well. This exists to help differentiate between messages, I think it'll make it easier for some plugins.
- `avatar_emotion` specifies what emotion the avatar will show while the command is being executed (i.e. while showing a message in the textbox or waiting for user input). The default emotion is "normal", and for the full list you can look at `res/kurisu/` for now, but I will probably reorganize it a little.
- `textbox_text` specifies what message will be shown in the textbox.
- `canvas_mode` specifies what will be shown in the center of the screen, I left it as an empty object above because it needs to be explained separately.

### Canvas modes

Canvas modes are what's shown in the center of the screen.

Right now there are 2 of them:

---

#### InputText

Shows an input field, where the user can type in text.

```json
{
  "canvas_mode": "InputText"
}
```

#### Select

Shows a list of options, and the user can filter them by typing text and then select one of them.

```json
{
  "canvas_mode": {
    "Select": ["Option 1", "Option 2", "Option 3", "Option 4"]
  }
}
```

---

I will probably add more later on. I'm thinking of a list of buttons, an image or video, and maybe something like a layout which combines multiple? But those are the 2 most important ones, so I made them first.
