particlesJS("particles", {
    "particles": {
        "number": {
            "value": 200,
            "density": {
                "enable": true,
                "value_area": 789.1476416322727
            }
        },
        "color": {
            "value": "#FFFFFF"
        },
        "shape": {
            "type": "circle",
            "stroke": {
                "width": 0,
                "color": "#000000"
            },
            "polygon": {
                "nb_sides": 5
            }
        },
        "opacity": {
            "value": 0.8,
            "random": false,
            "anim": {
                "enable": true,
                "speed": 0.2,
                "opacity_min": 0.4,
                "sync": false
            }
        },
        "size": {
            "value": 2,
            "random": true,
            "anim": {
                "enable": true,
                "speed": 2,
                "size_min": 0,
                "sync": false
            }
        },
        "line_linked": {
            "enable": true,
            "distance": 110,
            "color": "#ffffff",
            "opacity": 0.1,
            "width": 0.5
        },
        "move": {
            "enable": true,
            "speed": 0.8,
            "direction": "none",
            "random": true,
            "straight": false,
            "out_mode": "out",
            "bounce": false,
            "attract": {
                "enable": false,
                "rotateX": 600,
                "rotateY": 1200
            }
        }
    },
    "interactivity": {
        "detect_on": "canvas",
        "events": {
            "onhover": {
                "enable": true,
                "mode": "bubble"
            },
            "onclick": {
                "enable": true,
                "mode": "push"
            },
            "resize": true
        },
        "modes": {
            "grab": {
                "distance": 400,
                "line_linked": {
                    "opacity": 1
                }
            },
            "bubble": {
                "distance": 83.91608391608392,
                "size": 1,
                "duration": 3,
                "opacity": 1,
                "speed": 3
            },
            "repulse": {
                "distance": 200,
                "duration": 0.4
            },
            "push": {
                "particles_nb": 4
            },
            "remove": {
                "particles_nb": 2
            }
        }
    },
    "retina_detect": false
});

function rand(min,max)
{
    return Math.floor(Math.random()*(max-min+1)+min);
}

function makeColor(min, max) {
    return "rgba(" + rand(min, max) + "," + rand(min, max) + "," + rand(min, max) + ",1)";
}

function getColors() {
    return {
        color1: makeColor(65, 85),
        color2: makeColor(45, 65),
        color3: makeColor(20, 45),
        color4: makeColor(10, 20),
        color5: makeColor(0, 10)
    };
}

function getGradient() {
    let colors = getColors();
    //return "linear-gradient(0deg, " + colors.color1 + " 0%, " + colors.color2 + " 25%, " + colors.color3 + " 50%, " + colors.color4 + " 75%, " + colors.color5 + " 100%)";
    return "linear-gradient(125deg, rgba(60,107,148,1) 0%, rgba(167,91,91,1) 50%, rgba(164,129,129,1) 100%)";
}

document.getElementsByTagName("body")[0].style.background =  getGradient();
