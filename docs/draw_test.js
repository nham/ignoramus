var canvas = new fabric.Canvas('c');

var circs = [];

var grid_start = { x: 50, y: 50 };
var coord_to_pos = function(i, j) {
    var h_spacing = 60;
    var v_spacing = 60;
    return { x: grid_start.x + h_spacing*i,
             y: grid_start.y + v_spacing*j
    };
};

var rows = 6;
var cols = 4;
for (var i = 0; i < rows; i++) {
    for (var j = 0; j < cols; j++) {
        var pos = coord_to_pos(i, j);
        circs.push(new fabric.Circle({
            radius: 3, fill: 'green', left: pos.x, top: pos.y
        }));
    }
}

for (var i = 0; i < circs.length; i++) {
    canvas.add(circs[i]);
}
