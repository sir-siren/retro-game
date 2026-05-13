use crate::engine::input::Key;
use crate::games::invaders::state::{Bullet, InvadersState};
use crate::games::rand::fast_rand;
use crate::types::geometry::Direction;

pub fn tick(state: &mut InvadersState) {
    if state.is_game_over || state.is_complete {
        return;
    }

    state.tick = state.tick.wrapping_add(1);
    move_bullets(state);
    if state.tick % march_interval(state) == 0 {
        march_aliens(state);
    }
    maybe_fire_alien_bullet(state);
    handle_hits(state);
    check_wave_clear(state);
    check_player_hit(state);
}

pub fn handle_input(state: &mut InvadersState, key: Key) {
    match key {
        Key::Dir(Direction::Left) => {
            state.player_x = state.player_x.saturating_sub(2).max(1);
        }
        Key::Dir(Direction::Right) => {
            let max_x = i32::from(state.bounds.width.saturating_sub(2));
            state.player_x = state.player_x.saturating_add(2).min(max_x);
        }
        Key::Action if state.player_bullets.len() < max_player_bullets(state) => {
            state.player_bullets.push(Bullet {
                x: state.player_x,
                y: i32::from(state.bounds.height.saturating_sub(4)),
            });
        }
        _ => {}
    }
}

fn march_interval(state: &InvadersState) -> u64 {
    24u64.saturating_sub(u64::from(state.level.0) * 2).max(6)
}

const fn max_player_bullets(state: &InvadersState) -> usize {
    if state.level.0 >= 6 {
        3
    } else if state.level.0 >= 3 {
        2
    } else {
        1
    }
}

fn move_bullets(state: &mut InvadersState) {
    for bullet in &mut state.player_bullets {
        bullet.y -= 1;
    }
    for bullet in &mut state.alien_bullets {
        bullet.y += 1;
    }
    state.player_bullets.retain(|bullet| bullet.y > 1);
    state
        .alien_bullets
        .retain(|bullet| bullet.y < i32::from(state.bounds.height.saturating_sub(1)));
}

fn march_aliens(state: &mut InvadersState) {
    let should_drop = state.aliens.iter().filter(|a| a.is_alive).any(|alien| {
        let next_x = alien.x + state.direction;
        next_x <= 1 || next_x >= i32::from(state.bounds.width.saturating_sub(2))
    });

    if should_drop {
        state.direction *= -1;
        for alien in &mut state.aliens {
            if alien.is_alive {
                alien.y += 1;
            }
        }
    } else {
        for alien in &mut state.aliens {
            if alien.is_alive {
                alien.x += state.direction;
            }
        }
    }

    if state
        .aliens
        .iter()
        .any(|alien| alien.is_alive && alien.y >= i32::from(state.bounds.height.saturating_sub(5)))
    {
        state.is_game_over = true;
    }
}

fn maybe_fire_alien_bullet(state: &mut InvadersState) {
    let interval = 34u64.saturating_sub(u64::from(state.level.0) * 3).max(10);
    if state.tick % interval != 0 {
        return;
    }

    let living: Vec<_> = state
        .aliens
        .iter()
        .filter(|alien| alien.is_alive)
        .copied()
        .collect();
    if living.is_empty() {
        return;
    }
    let idx = usize::try_from(fast_rand(state.tick) % living.len() as u64).unwrap_or(0);
    let alien = living[idx];
    state.alien_bullets.push(Bullet {
        x: alien.x,
        y: alien.y + 1,
    });
}

fn handle_hits(state: &mut InvadersState) {
    let player_bullets = std::mem::take(&mut state.player_bullets);
    let mut remaining_bullets = Vec::with_capacity(player_bullets.len());
    for bullet in player_bullets {
        if hit_alien(state, bullet) || hit_shield(state, bullet) {
            continue;
        }
        remaining_bullets.push(bullet);
    }
    state.player_bullets = remaining_bullets;

    let incoming_bullets = std::mem::take(&mut state.alien_bullets);
    let mut alien_bullets = Vec::with_capacity(incoming_bullets.len());
    for bullet in incoming_bullets {
        if hit_shield(state, bullet) {
            continue;
        }
        alien_bullets.push(bullet);
    }
    state.alien_bullets = alien_bullets;
}

fn hit_alien(state: &mut InvadersState, bullet: Bullet) -> bool {
    for alien in &mut state.aliens {
        if alien.is_alive && (bullet.x - alien.x).abs() <= 1 && bullet.y == alien.y {
            alien.is_alive = false;
            let points = match alien.row {
                0 => 30,
                1 | 2 => 20,
                _ => 10,
            };
            state.score.0 = state.score.0.saturating_add(points);
            return true;
        }
    }
    false
}

fn hit_shield(state: &mut InvadersState, bullet: Bullet) -> bool {
    for shield in &mut state.shields {
        if shield.hp > 0 && shield.x == bullet.x && shield.y == bullet.y {
            shield.hp = shield.hp.saturating_sub(1);
            return true;
        }
    }
    false
}

fn check_wave_clear(state: &mut InvadersState) {
    if state.aliens.iter().any(|alien| alien.is_alive) {
        return;
    }

    state.level.0 = state.level.0.saturating_add(1).min(10);
    state.spawn_wave();
}

fn check_player_hit(state: &mut InvadersState) {
    let player_y = i32::from(state.bounds.height.saturating_sub(3));
    if state
        .alien_bullets
        .iter()
        .any(|bullet| (bullet.x - state.player_x).abs() <= 1 && bullet.y >= player_y)
    {
        state.is_game_over = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::TerminalSize;

    #[test]
    fn player_bullet_destroying_alien_awards_points() {
        let mut state = InvadersState::new(TerminalSize {
            width: 80,
            height: 24,
        });
        let alien = state.aliens[0];
        let bullet = Bullet {
            x: alien.x,
            y: alien.y,
        };

        assert!(hit_alien(&mut state, bullet));
        assert!(!state.aliens[0].is_alive);
        assert_eq!(state.score.0, 30);
    }
}
