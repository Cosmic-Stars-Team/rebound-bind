use rebound_bind::{
    REB_STATUS_REB_STATUS_SUCCESS, reb_particle, reb_simulation_add, reb_simulation_create,
    reb_simulation_energy, reb_simulation_free, reb_simulation_integrate,
    reb_simulation_move_to_com,
};

fn particle(m: f64, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> reb_particle {
    reb_particle {
        x,
        y,
        z,
        vx,
        vy,
        vz,
        ax: 0.0,
        ay: 0.0,
        az: 0.0,
        m,
        r: 0.0,
        name: std::ptr::null(),
        ap: std::ptr::null_mut(),
        sim: std::ptr::null_mut(),
    }
}

#[test]
fn creates_default_simulation() {
    unsafe {
        let sim = reb_simulation_create();
        assert!(!sim.is_null());

        assert_eq!((*sim).N, 0);
        assert_eq!((*sim).t, 0.0);
        assert_eq!((*sim).G, 1.0);
        assert_eq!((*sim).dt, 0.001);

        reb_simulation_free(sim);
    }
}

#[test]
fn adds_particles_and_updates_parent_pointer() {
    unsafe {
        let sim = reb_simulation_create();
        assert!(!sim.is_null());

        reb_simulation_add(sim, particle(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        reb_simulation_add(sim, particle(0.001, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0));

        assert_eq!((*sim).N, 2);
        assert!(!(*sim).particles.is_null());
        assert_eq!((*(*sim).particles.add(0)).sim, sim);
        assert_eq!((*(*sim).particles.add(1)).sim, sim);
        assert_eq!((*(*sim).particles.add(1)).x, 1.0);

        reb_simulation_free(sim);
    }
}

#[test]
fn integrates_simple_two_body_system() {
    unsafe {
        let sim = reb_simulation_create();
        assert!(!sim.is_null());

        (*sim).dt = 0.01;

        reb_simulation_add(sim, particle(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        reb_simulation_add(sim, particle(0.001, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0));
        reb_simulation_move_to_com(sim);

        let initial_energy = reb_simulation_energy(sim);
        let status = reb_simulation_integrate(sim, 0.1);
        let final_energy = reb_simulation_energy(sim);

        assert_eq!(status, REB_STATUS_REB_STATUS_SUCCESS);
        assert!(((*sim).t - 0.1).abs() < 1e-12);
        assert!((*sim).steps_done > 0);
        assert!((final_energy - initial_energy).abs() < 1e-8);

        reb_simulation_free(sim);
    }
}
