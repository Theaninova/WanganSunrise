import json
import bpy
import math

scale = 0.0001

bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete()

with open(r'YOUR_FILE.json', 'r') as f:
    data = json.load(f)

vertices = []
faces = []
i = 0

for section in data['sections']:
    x, y = section['position']
    nx, ny = section['normal']
    left = section['left']
    right = section['right']
    
    vertices.append(((x + nx * right) * scale, (y + ny * right) * scale, section['height'] * scale))
    vertices.append(((x + nx * left) * scale, (y + ny * left) * scale, section['height'] * scale))
    
    if i != 0:
        faces.append((i - 1, i - 2, i, i + 1))
    
    i += 2
    
mesh = bpy.data.meshes.new("mesh")
mesh.from_pydata(vertices, [], faces)
obj = bpy.data.objects.new(data['name'], mesh)

bpy.context.scene.collection.objects.link(obj)

# ====================
# Bank
# ====================

closed = True
for (n, p) in [('left', -1), ('right', 1)]:
    vertices = []
    faces = []
    i = 0

    for bank in data['bank_' + n]:
        section = data['sections'][bank['section']]
        x, y = section['position']
        nx, ny = section['normal']
        left = section[n]

        # I don't think this is quite right yet
        distance = bank['width']
    
        vertices.append((
            (x + nx * left) * scale,
            (y + ny * left) * scale,
            section['height'] * scale))
        vertices.append((
            (x + nx * (left - distance)) * scale,
            (y + ny * (left - distance)) * scale,
            (section['height'] - distance * p * bank['bank']) * scale))
    
        if bank['connect']:
            faces.append((i, i + 1, i + 3, i + 2))
    
        i += 2
    
    mesh = bpy.data.meshes.new("mesh")
    mesh.from_pydata(vertices, [], faces)
    obj = bpy.data.objects.new(data['name'] + '_bank_' + n, mesh)

    bpy.context.scene.collection.objects.link(obj)
