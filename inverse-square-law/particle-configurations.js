export default [
    {
        name: "Two rotating particles",
        speed: 0.1,
        k: 1.0,
        particles: [
            {
                radius: 5.,
                pos: { x: -30., y:  0.0 },
                vel: { x:   0., y: -1.0 },
                charge: 10.,
                mass: 1.
            },
            {
                radius: 5.,
                pos: { x: 30., y: 0.0 },
                vel: { x:  0., y: 1.0 },
                charge: -10.,
                mass: 1.
            }
        ]
    },
    {
        name: "Sun-Earth-Moon",
        speed: 0.1,
        k: -1.0,
        particles: [
            {
                radius: 25.,
                pos: { x: -2.0, y: 0.0 },
                vel: { x: 0.0, y: 0.2213928691 },
                charge: 100000.0,
                mass: 100000.0
            },
            {
                radius: 10.,
                pos: { x: 200.0, y: 0.0 },
                vel: { x: 0.0, y: -22.13928691 },
                charge: 1000.0,
                mass: 1000.0
            },
            {
                radius: 5.,
                pos: { x: 210.0, y: 0.0 },
                vel: { x: 0.0, y: -32.04027701},
                charge: 10.0,
                mass: 10.0
            }
        ]
    },
    {
        name: "Sun-Earth-Moon2",
        speed: 0.001,
        k: -0.001,
        particles: [
            {
                radius: 25.,
                pos: { x: -0.2002, y: 0.0 },
                vel: { x: 0.0, y: 7.071060748e-3 },
                charge: 10000000.0,
                mass: 10000000.0
            },
            {
                radius: 10.,
                pos: { x: 199.99, y: 0.0 },
                vel: { x: 0.0, y: -7.06299775 },
                charge: 10000.0,
                mass: 10000.0
            },
            {
                radius: 5.,
                pos: { x: 210.0, y: 0.0 },
                vel: { x: 0.0, y: -8.06299775},
                charge: 10.0,
                mass: 10.0
            }
        ]
    },
    {
        name: "Sun-Earth-Moon3",
        speed: 0.05,
        k: -0.1,
        particles: [
            {
                radius: 25.,
                pos: { x: -0.2002, y: 0.0 },
                vel: { x: 0.0, y: 0.00040000799616024324 },
                charge: 125000.0,
                mass: 125000.0
            },
            {
                radius: 5.,
                pos: { x: 499.999, y: 0.0 },
                vel: { x: 0.0, y: -4.99996838038536 },
                charge: 10.0,
                mass: 10.0
            },
            {
                radius: 2.,
                pos: { x: 510.0, y: 0.0 },
                vel: { x: 0.0, y: -5.316196146402198 },
                charge: 0.001,
                mass: 0.001
            }
        ]
    },
    {
        name: "stable arrangement",
        speed: 0.001,
        k: 1.0,
        particles: [
            {
                radius: 10.,
                pos: { x: -30., y: -30. },
                vel: { x: 0., y: 0. },
                charge: 2.,
                mass: 10.
            },
            {
                radius: 10.,
                pos: { x: -30., y: 30. },
                vel: { x: 0., y: 0. },
                charge: 2.,
                mass: 10.
            },
            {
                radius: 10.,
                pos: { x: 30., y: -30. },
                vel: { x: 0., y: 0. },
                charge: 2.,
                mass: 10.
            },
            {
                radius: 10.,
                pos: { x: 30., y: 30. },
                vel: { x: 0., y: 0. },
                charge: 2.,
                mass: 10.
            },
            {
                radius: 12.07,
                pos: { x: 0., y: 0. },
                vel: { x: 0., y: 0. },
                charge: -2.414213562373095,
                mass: 10.
            }
        ]
    },
    {
        name: "double star + planet",
        speed: 0.005,
        k: -0.1,
        particles: [
            {
                radius:	12.,
                pos: { x: -15., y: 0.0 },
                vel: { x: 0.0, y: -10.206207261596576 },
                charge: 62500.0,
                mass: 62500.0
            },
            {
                radius:	12.,
                pos: { x: 15., y: 0.0 },
                vel: { x: 0.0, y: 10.206207261596576 },
                charge: 62500.0,
                mass: 62500.0
            },
            {
                radius: 5.,
                pos: { x: 499.999, y: 0.0 },
                vel: { x: 0.0, y: -4.99996838038536 },
                charge: 10.0,
                mass: 10.0
            }
        ]
    },
    {
        name: "almost collision",
        speed: 0.004,
        k: 1.,
        particles: [
            {
                radius:	20.,
                pos: { x: 50., y: 0.0 },
                vel: { x: 0.0, y: 0.0 },
                charge: 15.0,
                mass: 100000000000000.0
            },
            {
                radius:	5.,
                pos: { x: -400., y: -20.0 },
                vel: { x: 1., y: 0. },
                charge: 1.0,
                mass: 1.0
            }
        ]
    }
]