use std::cmp::min;

use macroquad::prelude::*;
use macroquad::audio::*;
use crate::enemy::*;
use crate::game::*;
use crate::bullet::*;

impl Game {
    // player bullet collides with enemy
    pub fn bullet_enemy_coll(&mut self, b: &mut Bullet) {
        let mut enemies = std::mem::take(&mut self.enemies);
        for enemy in enemies.iter_mut() {
            let hit = rect_collide(
                Rect{
                    x: b.x - b.size,
                    y: b.y - b.size,
                    w: b.size * 2.0,
                    h: b.size * 2.0,
                },enemy.get_rect()); 
            
            if hit {
                b.hit = hit;
                enemy.health -= self.player.damage;
            }
        }
        self.enemies = enemies;
    }

    // enemy bullet collides with player
    pub fn bullet_player_coll(&mut self, b: &mut Bullet) {
        let hit = rect_collide(
            self.player.get_rect(),
            Rect{
                x: b.x - b.size,
                y: b.y - b.size,
                w: b.size * 2.0,
                h: b.size * 2.0,
            }
        );

        if hit {
            b.hit = hit;
            if b.state == self.color_state {
                self.player.health = min(self.player.max_health, self.player.health + self.player.heal_from_b)
            } else {
                self.player.health -= b.damage;
                play_sound(&self.assets.hit, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
            }
        }
    }
    // =========== ENEMIES =============

    // enemy and player collision
    pub fn enemy_collision(&mut self,e: &mut Enemy) {
        let hit = rect_collide(e.get_rect(), self.player.get_rect());



        if hit && e.can_collide {
            if e.kind == EnemyType::FollowEnemy {
                if e.state == self.color_state {
                self.player.health = min(self.player.max_health, self.player.health + self.player.heal_from_b)
                } else {
                    self.player.health -= 1;
                }
            } else {
                self.player.health -= e.contact_damage;
                play_sound(&self.assets.hit, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
            }
            e.health = 0.0;
        }
    }
}