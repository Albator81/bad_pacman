import pygame as pg
from settings import *
from random import random
from map import mini_map

def collides(tile):
    return tile[0] >= MAP_WIDTH or tile[1] >= MAP_HEIGHT or mini_map[tile[1]][tile[0]] == 1

def reverse_direction(direction):
    if direction == DIRECTION_UP:
        return DIRECTION_DOWN
    elif direction == DIRECTION_LEFT:
        return DIRECTION_RIGHT
    elif direction == DIRECTION_RIGHT:
        return DIRECTION_LEFT
    else:
        return DIRECTION_UP
def adjacent_directions(direction):
    if direction == DIRECTION_UP:
        return (DIRECTION_RIGHT, DIRECTION_LEFT)
    elif direction == DIRECTION_LEFT:
        return (DIRECTION_UP, DIRECTION_DOWN)
    elif direction == DIRECTION_RIGHT:
        return (DIRECTION_DOWN, DIRECTION_UP)
    else:
        return (DIRECTION_LEFT, DIRECTION_RIGHT)

def left_tile(tile):
    return (tile[0] - 1, tile[1])
def right_tile(tile):
    return (tile[0] + 1, tile[1])
def top_tile(tile):
    return (tile[0], tile[1] - 1)
def bottom_tile(tile):
    return (tile[0], tile[1] + 1)

def dist(tile1, tile2):
    return math.dist(tile1, tile2)
def fake_dist(tile1, tile2):
    return (tile1[0] - tile2[0]) * (tile1[0] - tile2[0]) + (tile1[1] - tile2[1]) * (tile1[1] - tile2[1])
def random_bool():
    return random() < 0.5

def tile_nearer_than(tile1, tile2, target_tile):
    return fake_dist(tile1, target_tile) < fake_dist(tile2, target_tile)

def to_next_tile(tile, direction):
    if direction == DIRECTION_UP:
        return top_tile(tile)
    elif direction == DIRECTION_LEFT:
        return left_tile(tile)
    elif direction == DIRECTION_RIGHT:
        return right_tile(tile)
    else:
        return bottom_tile(tile)

def pinky_tt(pacman_tile, pacman_direction):
    if pacman_direction == DIRECTION_UP:
        return (pacman_tile[0] - 4, pacman_tile[1] + 4)
    elif pacman_direction == DIRECTION_LEFT:
        return (pacman_tile[0] - 4, pacman_tile[1])
    elif pacman_direction == DIRECTION_RIGHT:
        return (pacman_tile[0] + 4, pacman_tile[1])
    else:
        return (pacman_tile[0], pacman_tile[1] + 4)
def inky_tt(pacman_tile, pacman_direction, blinky_tile):
    if pacman_direction == DIRECTION_UP:
        tt = (pacman_tile[0] - 2, pacman_tile[1] + 2)
    elif pacman_direction == DIRECTION_LEFT:
        tt = (pacman_tile[0] - 2, pacman_tile[1])
    elif pacman_direction == DIRECTION_RIGHT:
        tt = (pacman_tile[0] + 2, pacman_tile[1])
    else:
        tt = (pacman_tile[0], pacman_tile[1] + 2)
    return (tt[0] - blinky_tile[0]) * 2, (tt[1] - blinky_tile[1]) * 2
def clyde_tt(pacman_tile, clyde_tile):
    if dist(clyde_tile, pacman_tile) < 10:
        return pacman_tile
    else:
        return CLYDE_HOME_TILE

def to_direction(tile, next_tile):
    if tile[0] == next_tile[0]:
        if tile[1] == next_tile[1]:
            raise ValueError('this aint possible')
        elif tile[1] < next_tile[1]: return DIRECTION_DOWN
        else: return DIRECTION_UP
    elif tile[0] < next_tile[0]: return DIRECTION_RIGHT
    else: return DIRECTION_LEFT

def nearer_tile(tile1, tile2, target_tile):
    if tile_nearer_than(tile1, tile2, target_tile):
        return tile1
    return tile2

def next_best_direction(tile, direction1, target_tile):
    direction2, direction3 = adjacent_directions(direction1)

    tile1 = to_next_tile(tile, direction1)
    tile2 = to_next_tile(tile, direction2)
    tile3 = to_next_tile(tile, direction3)

    c1 = collides(tile1)
    c2 = collides(tile2)
    c3 = collides(tile3)

    if (c1, c2, c3) == (False, True , True ): return direction1
    if (c1, c2, c3) == (True , False, True ): return direction2
    if (c1, c2, c3) == (True , True , False): return direction3
    if (c1, c2, c3) == (False, False, True ): return to_direction(tile, nearer_tile(tile1, tile2, target_tile))
    if (c1, c2, c3) == (False, True , False): return to_direction(tile, nearer_tile(tile1, tile3, target_tile))
    if (c1, c2, c3) == (True , False, False): return to_direction(tile, nearer_tile(tile2, tile3, target_tile))
    if (c1, c2, c3) == (False, False, False): return to_direction(tile, nearer_tile(nearer_tile(tile1, tile2, target_tile), tile3, target_tile))
    if (c1, c2, c3) == (True , True , True ): return reverse_direction(direction1) # shouldn't happen tho

def random_direction(*directions):
    return directions[int(random()*len(directions))]

def next_random_direction(tile, direction1):
    direction2, direction3 = adjacent_directions(direction1)

    tile1 = to_next_tile(tile, direction1)
    tile2 = to_next_tile(tile, direction2)
    tile3 = to_next_tile(tile, direction3)

    c1 = collides(tile1)
    c2 = collides(tile2)
    c3 = collides(tile3)

    if (c1, c2, c3) == (False, True , True ): return direction1
    if (c1, c2, c3) == (True , False, True ): return direction2
    if (c1, c2, c3) == (True , True , False): return direction3
    if (c1, c2, c3) == (False, False, True ): return random_direction(direction1, direction2)
    if (c1, c2, c3) == (False, True , False): return random_direction(direction1, direction3)
    if (c1, c2, c3) == (True , False, False): return random_direction(direction2, direction3)
    if (c1, c2, c3) == (False, False, False): return random_direction(direction1, direction2, direction3)
    if (c1, c2, c3) == (True , True , True ): return reverse_direction(direction1) # shouldn't happen tho
    print('!')

def need_direction_update(tile, direction1) -> bool:
    direction2, direction3 = adjacent_directions(direction1)

    tile1 = to_next_tile(tile, direction1)
    tile2 = to_next_tile(tile, direction2)
    tile3 = to_next_tile(tile, direction3)

    c1 = collides(tile1)
    c2 = collides(tile2)
    c3 = collides(tile3)

    return c1 or (not c2) or (not c3)

class Ghost:
    def __init__(self, game, tile=[1., 1.5], mode=MODE_SCATTER, direction=DIRECTION_UP) -> None:
        self.game = game
        self.tile = [int(tile[0]), int(tile[1])]
        self.x, self.y = tile
        self.x += 0.5
        self.y += 0.5
        self.mode = mode
        self.direction = direction
        self.target_tile = [0, 0]
        self.locked = True
        self.decision_taken = False

    def unleash(self):
        self.locked = False

    def chase_move(self): # useless
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile)
    def frightened_move(self): # useless
        self.direction = next_random_direction(self.tile, self.direction)
    def scatter_move(self): # useless
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile)

    def movement(self):
        if self.direction == DIRECTION_UP:
            self.y -= GHOST_SPEED * self.game.delta_time
        elif self.direction == DIRECTION_DOWN:
            self.y += GHOST_SPEED * self.game.delta_time
        elif self.direction == DIRECTION_RIGHT:
            self.x += GHOST_SPEED * self.game.delta_time
        elif self.direction == DIRECTION_LEFT:
            self.x -= GHOST_SPEED * self.game.delta_time
        else:
            raise ValueError('direction not definied: '+str(self.direction))

    def tile_movement(self):
        """important: all 0.5 are added because x and y are at the center of the ghost"""
        if self.tile[0] + 1.5 <= self.x:
            self.tile[0] += 1
            self.x = int(self.x) + 0.5
            self.decision_taken = False
        elif self.tile[0] - 0.5 >= self.x:
            self.tile[0] -= 1
            self.x = int(self.x)
            self.x += 0.5 if self.x > 0 else -0.5
            self.decision_taken = False

        if self.tile[1] + 1.5 <= self.y:
            self.tile[1] += 1
            self.y = int(self.y) + 0.5
            self.decision_taken = False
        elif self.tile[1] - 0.5 >= self.y:
            self.tile[1] -= 1
            self.y = int(self.y)
            self.y += 0.5 if self.y > 0 else -0.5
            self.decision_taken = False

    def update_pos(self):
        self.movement()
        self.tile_movement()

    def draw(self):
        pg.draw.circle(self.game.screen, (0, 0, 255), (self.x * RESIZE, self.y * RESIZE), 6)

class Blinky(Ghost):
    def __init__(self, game, tile=BLINKY_POS, mode=MODE_SCATTER, direction=DIRECTION_UP) -> None:
        super().__init__(game, tile, mode, direction)

    def update_direction(self, pacman_tile):
        if self.locked:
            self.direction = reverse_direction(self.direction)
            return

        if self.mode == MODE_CHASE:
            self.target_tile = pacman_tile
            self.chase_move()
        elif self.mode == MODE_SCATTER:
            self.target_tile = BLINKY_HOME_TILE
            self.scatter_move()
        elif self.mode == MODE_FRIGHTENED:
            self.frightened_move()
        else:
            raise ValueError(f'Mode: {self.mode}')

    def update(self, pacman_tile, force_update=False):
        super().update_pos()
        if not self.decision_taken and (force_update or need_direction_update(self.tile, self.direction)):
            self.decision_taken = True
            self.update_direction(pacman_tile) # now it only updates at crossways

    def draw(self):
        pg.draw.circle(self.game.screen, (255, 0, 0), (self.x * RESIZE, self.y * RESIZE), 6)

class Pinky(Ghost):
    def __init__(self, game, tile=PINKY_POS, mode=MODE_SCATTER, direction=DIRECTION_UP) -> None:
        super().__init__(game, tile, mode, direction)

    def update_direction(self, pacman_tile, pacman_direction):
        if self.locked:
            self.direction = reverse_direction(self.direction)
            return

        if self.mode == MODE_CHASE:
            self.target_tile = pinky_tt(pacman_tile, pacman_direction)
            self.chase_move()
        elif self.mode == MODE_SCATTER:
            self.target_tile = PINKY_HOME_TILE
            self.scatter_move()
        elif self.mode == MODE_FRIGHTENED:
            self.frightened_move()
        else:
            raise ValueError(f'Mode: {self.mode}')

    def update(self, pacman_tile, pacman_direction, force_update=False):
        super().update_pos()
        if not self.decision_taken and (force_update or need_direction_update(self.tile, self.direction)):
            self.decision_taken = True
            self.update_direction(pacman_tile, pacman_direction) # now it only updates at crossways

    def draw(self):
        pg.draw.circle(self.game.screen, (255, 184, 255), (self.x * RESIZE, self.y * RESIZE), 6)

class Inky(Ghost):
    def __init__(self, game, tile=INKY_POS, mode=MODE_SCATTER, direction=DIRECTION_UP) -> None:
        super().__init__(game, tile, mode, direction)

    def update_direction(self, pacman_tile, pacman_direction, blinky_tile):
        if self.locked:
            self.direction = reverse_direction(self.direction)
            return

        if self.mode == MODE_CHASE:
            self.target_tile = inky_tt(pacman_tile, pacman_direction, blinky_tile)
            self.chase_move()
        elif self.mode == MODE_SCATTER:
            self.target_tile = INKY_HOME_TILE
            self.scatter_move()
        elif self.mode == MODE_FRIGHTENED:
            self.frightened_move()
        else:
            raise ValueError(f'Mode: {self.mode}')

    def update(self, pacman_tile, pacman_direction, blinky_tile, force_update=False):
        super().update_pos()
        if not self.decision_taken and (force_update or need_direction_update(self.tile, self.direction)):
            self.decision_taken = True
            self.update_direction(pacman_tile, pacman_direction, blinky_tile) # now it only updates at crossways

    def draw(self):
        pg.draw.circle(self.game.screen, (0, 255, 255), (self.x * RESIZE, self.y * RESIZE), 6)

class Clyde(Ghost):
    def __init__(self, game, tile=CLYDE_POS, mode=MODE_SCATTER, direction=DIRECTION_UP) -> None:
        super().__init__(game, tile, mode, direction)

    def update_direction(self, pacman_tile):
        if self.locked:
            self.direction = reverse_direction(self.direction)
            return

        if self.mode == MODE_CHASE:
            self.target_tile = clyde_tt(pacman_tile, self.tile)
            self.chase_move()
        elif self.mode == MODE_SCATTER:
            self.target_tile = CLYDE_HOME_TILE
            self.scatter_move()
        elif self.mode == MODE_FRIGHTENED:
            self.frightened_move()
        else:
            raise ValueError(f'Mode: {self.mode}')

    def update(self, pacman_tile, force_update=False):
        super().update_pos()
        if not self.decision_taken and (force_update or need_direction_update(self.tile, self.direction)):
            self.decision_taken = True
            self.update_direction(pacman_tile) # now it only updates at crossways

    def draw(self):
        pg.draw.circle(self.game.screen, (255, 184, 82), (self.x * RESIZE, self.y * RESIZE), 6)
