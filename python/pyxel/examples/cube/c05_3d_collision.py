import pyxel
from pyxel.cube import Camera, Collider, Mat4, Mesh, Node, Quat, Shading, Vec3

GRAVITY = -0.45
JUMP_SPEED = 6.5
MOVE_SPEED = 2.0

CAMERA_MIN_DISTANCE = 48


class MovingPlatform(Node):
    def __init__(self, start, end):
        super().__init__()

        self.add_child(Node.from_mesh(Mesh.from_glb("assets/moving_platform.glb")))
        self.collider = Collider(
            mesh=Mesh.from_glb("assets/moving_platform_collision.glb"), mass=0
        )
        self.tags = ["ground"]
        self.start = start
        self.end = end
        self.transform = Mat4.from_translation(start)
        self.delta = Vec3.ZERO

    def on_update(self):
        pos = self.start.lerp(self.end, (1 - pyxel.cos(pyxel.frame_count * 1.5)) / 2)
        self.delta = pos - self.transform.pos
        self.transform = Mat4.from_translation(pos)


class Player(Node):
    def __init__(self, start):
        super().__init__()

        self.start = start + Vec3(0, 8, 0)
        self.collider = Collider(radius=8, mass=1)
        self.coins = 0
        self.reset()

        mesh = Mesh.from_glb("assets/player.glb")
        self.motions = {motion.name: motion for motion in mesh.motions}
        self.actor = Node.from_mesh(mesh)
        self.add_child(self.actor)

    def reset(self):
        self.transform = Mat4.from_translation(self.start)
        self.collider.velocity = Vec3.ZERO
        self.on_floor = True
        self.floor_normal = Vec3.UP
        self.platform = None
        self.jumps = 0

        self.move_input = (0, 0)
        self.move = Vec3.ZERO
        self.heading = 180
        self.rotation = Quat.from_euler(Vec3(0, self.heading, 0))
        self.motion = None

    def on_update(self):
        if self.transform.pos.y < -100:
            self.reset()
            self.parent.reset_camera()

        # Move with the platform before checking contacts.
        if self.platform:
            self.transform = self.transform.translate(self.platform.delta)
        self.platform = None

        # Keep the movement direction steady while the camera turns.
        dx = pyxel.btn(pyxel.KEY_RIGHT) - pyxel.btn(pyxel.KEY_LEFT)
        dz = pyxel.btn(pyxel.KEY_UP) - pyxel.btn(pyxel.KEY_DOWN)
        if (dx, dz) != self.move_input:
            camera = self.parent.camera.transform
            right = Vec3(camera[0, 0], 0, camera[2, 0]).normalize()
            forward = Vec3(-camera[0, 2], 0, -camera[2, 2]).normalize()
            self.move = right * dx + forward * dz
            self.move_input = (dx, dz)
        move = self.move.normalize() * MOVE_SPEED
        if move.length() > 0:
            self.heading = pyxel.atan2(move.x, move.z)

        vy = max(self.collider.velocity.y + GRAVITY, -10)
        if self.on_floor:
            # Follow the slope to stay grounded when moving downhill.
            vy = min(-move.dot(self.floor_normal) / self.floor_normal.y, 0) + GRAVITY

        if pyxel.btnp(pyxel.KEY_SPACE) and self.jumps < 2:
            vy = JUMP_SPEED
            if self.jumps == 0:
                self.parent.yaw = self.heading + 180
            self.jumps += 1
            self.motion = None
            self.on_floor = False
            pyxel.play(0, 0)

        self.collider.velocity = Vec3(move.x, vy, move.z)
        self.animate(move.length() > 0)

        # on_collide restores ground contact after this update.
        self.on_floor = False

    def animate(self, walking):
        self.rotation = self.rotation.slerp(
            Quat.from_euler(Vec3(0, self.heading, 0)), 0.25
        )
        self.actor.transform = Mat4.from_quat(self.rotation).translate(Vec3(0, -8, 0))

        motion = "jump" if not self.on_floor else "walk" if walking else "idle"
        if motion != self.motion:
            self.actor.play_motion(self.motions[motion])
            self.motion = motion

    def on_collide(self, other, contact):
        if "coin" in other.tags:
            other.active = other.visible = False
            self.coins += 1
            pyxel.play(1, 1)
            return

        if other is self.parent.goal:
            other.active = False
            pyxel.play(2, 2)
            return

        # Push out of solid surfaces without canceling an upward jump.
        self.transform = self.transform.translate(contact.normal * contact.depth)

        if contact.normal.y > 0.45 and self.collider.velocity.y <= 0:
            self.on_floor = True
            self.floor_normal = contact.normal
            self.jumps = 0
            v = self.collider.velocity
            self.collider.velocity = Vec3(v.x, 0, v.z)
            if isinstance(other, MovingPlatform):
                self.platform = other

    def on_draw(self):
        hit = self.parent.raycast(self.transform.pos, Vec3.DOWN, tags=["ground"])
        if hit:
            self.shaded(False)
            # Leave room for the downhill side of the shadow on a slope.
            self.decal(hit.distance + 8)
            mat = Mat4.from_euler(Vec3(-90, 0, 0))
            self.dither(0.5)
            self.elli(mat, 14, 14, 0)
            self.dither(1)
            self.elli(mat, 9, 9, 0)


class App(Node):
    def __init__(self):
        super().__init__()

        pyxel.init(320, 240, title="3D Collision")
        pyxel.Image.from_image("assets/garden.png", include_colors=True)

        pyxel.sounds[0].mml(
            "T240 Q100 @1 V64 O3 E32 @2 V112 @ENV1{127,6,96,30,0} @GLI1{-1900,30} O5 C8."
        )
        pyxel.sounds[1].mml("T225 Q100 @2 V112 O5 A32 @ENV1{127,3,96,15,24,18,0} >E8.")
        pyxel.sounds[2].mml(
            "T200 Q80 @2 V104 O5 L16 E G >C8 R <G A8 B Q100 @ENV1{127,6,96,66,0} >C4."
        )

        self.lighting = Shading(pyxel.colors)
        self.lighting.direction = Vec3(-0.5, -1.0, 0.8)

        self.camera = Camera()
        self.camera.near = 2

        stage = Node.from_mesh(Mesh.from_glb("assets/garden.glb"))
        stage.collider = Collider(
            mesh=Mesh.from_glb("assets/garden_collision.glb"), mass=0
        )
        stage.tags = ["ground"]
        self.add_child(stage)

        # Placement markers are saved with the stage in the model editor.
        self.coins = stage.find_by_name("Coins")[0].children
        coin_mesh = Mesh.from_glb("assets/coin.glb")
        for coin in self.coins:
            coin.add_child(Node.from_mesh(coin_mesh))
            coin.collider = Collider(radius=5, trigger=True, mass=0)
            coin.tags = ["coin"]

        self.goal = stage.find_by_name("Goal")[0]
        self.goal.transform = self.goal.transform.translate(Vec3(0, 8, 0))
        self.goal.collider = Collider(size=Vec3(24, 16, 24), trigger=True, mass=0)

        start = stage.find_by_name("PlatformStart")[0].world_transform.pos
        end = stage.find_by_name("PlatformEnd")[0].world_transform.pos
        self.add_child(MovingPlatform(start, end))

        self.player = Player(stage.find_by_name("Start")[0].world_transform.pos)
        self.add_child(self.player)

        self.reset_camera()
        pyxel.run(self.update_game, self.draw_game)

    def on_update(self):
        for coin in self.coins:
            coin.children[0].transform = Mat4.IDENTITY.rotate_y(pyxel.frame_count * 4)

    def reset_camera(self):
        self.yaw = 28
        self.target = self.player.transform.pos
        self.camera_offset = None
        self.update_camera()

    def update_camera(self):
        focus = self.player.transform.pos
        self.target += (focus - self.target) * 0.15
        back = Vec3(pyxel.sin(self.yaw), 0, pyxel.cos(self.yaw))
        side = Vec3(back.z, 0, -back.x)
        if self.camera_offset and self.camera_offset.dot(side) < 0:
            side = -side

        # First move closer, then try above or beside the obstacle.
        offset = Vec3.ZERO
        for candidate in (
            self.target - focus + back * 100 + Vec3(0, 63, 0),
            back * 60 + Vec3(0, 60, 0),
            back * 30 + Vec3(0, 60, 0),
            back * 8 + Vec3(0, 60, 0),
            back * 80,
            (back + side) * 56,
            (back - side) * 56,
            -back * 80,
        ):
            candidate = self.clip_camera_offset(focus, candidate)
            if candidate.length() > offset.length():
                offset = candidate
            if offset.length() >= CAMERA_MIN_DISTANCE:
                break

        if self.camera_offset:
            previous = self.camera_offset
            turn = Quat.from_two_vectors(previous, offset)
            angle = Quat.IDENTITY.angle_to(turn)
            turn = Quat.IDENTITY.slerp(turn, min(0.25, 8 / max(angle, 0.001)))
            distance = previous.length() + (offset.length() - previous.length()) * 0.25
            offset = turn * previous.normalize() * max(CAMERA_MIN_DISTANCE, distance)
            clipped = self.clip_camera_offset(focus, offset)
            if clipped.length() >= CAMERA_MIN_DISTANCE:
                offset = clipped
            else:
                # Slide along the obstacle without moving inside the player.
                eye = focus + offset
                start = self.camera.transform.pos
                for _ in range(2):
                    delta = eye - start
                    hit = self.raycast(start, delta, delta.length(), tags=["ground"])
                    if hit is None:
                        break
                    eye += hit.normal * (4 - (eye - hit.point).dot(hit.normal))
                offset = eye - focus

        self.camera_offset = offset
        self.camera.transform = Mat4.look_at(focus + offset, focus)

    def clip_camera_offset(self, focus, offset):
        hit = self.raycast(focus, offset, offset.length(), tags=["ground"])
        if hit:
            return offset.normalize() * max(1, hit.distance - 6)
        return offset

    def update_game(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_S):
            self.shading = self.lighting if self.shading is None else None

        self.update()
        if self.player.on_floor and self.player.move.length() > 0:
            turn = (self.player.heading + 180 - self.yaw + 180) % 360 - 180
            self.yaw += pyxel.clamp(turn * 0.05, -2, 2)
        self.update_camera()

    def draw_game(self):
        pyxel.cls(30)
        self.draw(0, 0, pyxel.width, pyxel.height)

        self.draw_text(8, 8, f"COINS {self.player.coins}/{len(self.coins)}")
        if not self.goal.active:
            self.draw_text(252, 8, "SUMMIT REACHED!")

        pyxel.rect(0, 228, 320, 12, 0)
        help_text = "Arrows: Move  Space: Jump x2  S: Shading  Q: Quit"
        pyxel.text((320 - len(help_text) * 4) // 2, 231, help_text, 7)

    def draw_text(self, x, y, text):
        for dy in range(-1, 2):
            for dx in range(-1, 2):
                pyxel.text(x + dx, y + dy, text, 0)

        pyxel.text(x, y, text, 7)


App()
