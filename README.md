# 5deam

5deam is a 5beam client that allows for downloading BFDIA 5b levels without the use of HTML5b.

## Usage

This is a command-line tool that outputs level data for a given level id.

```
deam --level {{levelid}}
```

The level string can then be added manually to the levels.txt file. **Automatic saving to levels.txt will be added soon(TM)**

The levels.txt file also **must** be saved in the Windows-1252 encoding, or else the walkable wooden block will fail to render.