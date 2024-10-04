import pygame as pg
import sys
from settings import *
from map import *
from player import *
from ghost import *
from raycasting import *
from object_renderer import *
from sprite_object import *


class Game:
    def __init__(self):
        pg.init()
        pg.mouse.set_visible(False)
        self.screen = pg.display.set_mode(RES)
        self.clock = pg.time.Clock()
        self.delta_time = 1
        self.new_game()

    def new_game(self):
        self.map = Map(self)
        self.player = Player(self)
        self.blinky = Blinky(self)
        self.pinky = Pinky(self)
        self.inky = Inky(self)
        self.clyde = Clyde(self)
        self.blinky.unleash()
        self.pinky.unleash()
        self.inky.unleash()
        self.clyde.unleash()
        # self.object_renderer = ObjectRenderer(self)
        # self.raycasting = RayCasting(self)
        # self.food = load_food(self)

    def update(self):
        self.player.update()
        self.blinky.update(self.player.get_tile())
        self.pinky.update(self.player.get_tile(), DIRECTION_RIGHT)
        self.inky.update(self.player.get_tile(), DIRECTION_RIGHT, self.blinky.tile)
        self.clyde.update(self.player.get_tile())
        # self.raycasting.update()
        # for f in self.food:
        #     f.update()
        pg.display.flip()
        self.delta_time = self.clock.tick(FPS)
        pg.display.set_caption(f'{self.clock.get_fps() :.1f}')

    def draw(self):
        self.screen.fill((0, 0, 0))
        # self.clock.get_rawtime()
        # self.object_renderer.draw()
        # self.player.mouth()
        self.map.draw()
        # for f in self.food:
        #     f.draw()
        self.player.draw()
        self.blinky.draw()
        self.pinky.draw()
        self.inky.draw()
        self.clyde.draw()

    def check_events(self):
        event_list = pg.event.get()
        for event in event_list:
            if event.type == pg.QUIT or (event.type == pg.KEYDOWN and event.key == pg.K_ESCAPE):
                pg.quit()
                sys.exit()

    def run(self):
        while 1:
            self.check_events()
            self.update()
            self.draw()


if __name__ == '__main__':
    game = Game()
    game.run()
