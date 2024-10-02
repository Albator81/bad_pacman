import pygame as pg
import math


def floor_my(number, ndigits: int):
    sub_result = number * 10** ndigits // 1 * 10** -ndigits
    if ndigits <= 0:
        return int(sub_result)
    else:
        return float(format(sub_result, f'.{len(str(number))}f'))


pg.display.init()
iw, ih = pg.display.Info().current_w, pg.display.Info().current_h
RES = WIDTH, HEIGHT = floor_my(iw - (100 if iw > 100 else 0), -2), floor_my(ih - (100 if ih > 100 else 0), -2)
HALF_WIDTH = WIDTH // 2
HALF_HEIGHT = HEIGHT // 2
FPS = 300

RESIZE = (
            pg.display.Info().current_h // 40 # previously 31 # or 32 idkkkk
            if pg.display.Info().current_h < pg.display.Info().current_w or
            abs(pg.display.Info().current_h - pg.display.Info().current_w) < 40
            else
            pg.display.Info().current_w // 28
        )

PLAYER_POS = 1.5, 5
PLAYER_ANGLE = math.radians(-90)
PLAYER_SPEED = 0.004
PLAYER_ROT_SPEED = 0.002
PLAYER_SIZE_SCALE = 60

MOUSE_SENSITIVITY = 0.000_3
MOUSE_MAX_REL = 40
MOUSE_BORDER_LEFT = 200
MOUSE_BORDER_RIGHT = WIDTH - MOUSE_BORDER_LEFT

FLOOR_COLOR = (1, 0, 30)

FOV = math.pi / 3
HALF_FOV = FOV / 2
NUM_RAYS = WIDTH // 2
HALF_NUM_RAYS = NUM_RAYS // 2
DELTA_ANGLE = FOV / NUM_RAYS
MAX_DEPTH = 31

SCREEN_DIST = HALF_WIDTH / math.tan(HALF_FOV)
SCALE = WIDTH // NUM_RAYS

TEXTURE_SIZE = 256
HALF_TEXTURE_SIZE = TEXTURE_SIZE // 2

GHOST_DIRECTION = 0
BLINKY_POS = 11, 13
PINKY_POS = 12, 13
INKY_POS = 15, 13
CLYDE_POS = 16, 13

BLINKY_HOME_TILE = 25, -2
PINKY_HOME_TILE  = 2 , -2
INKY_HOME_TILE   = 27, 33
CLYDE_HOME_TILE  = 0 , 33

ENERGIZER_POS = ((1, 1), (26, 1), (1, 29), (26, 29))
GHOST_HOME_POS = (
                        (13, 12), (14, 12),
    (11, 13), (12, 13), (13, 13), (14, 13), (15, 13), (16, 13),
    (11, 14), (12, 14), (13, 14), (14, 14), (15, 14), (16, 14),
    (11, 15), (12, 15), (13, 15), (14, 15), (15, 15), (16, 15),
)

MODE_SCATTER    = 0
MODE_CHASE      = 1
MODE_FRIGHTENED = 2

DIRECTION_UP    = 0
DIRECTION_DOWN  = 1
DIRECTION_RIGHT = 2
DIRECTION_LEFT  = 3

MAP_WIDTH = 28
MAP_HEIGHT = 30

GHOST_SPEED = 0.0045