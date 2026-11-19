from pathlib import Path
import sys,time,json,statistics
sys.path.insert(0,sys.argv[1])
import pyxel as p
p.init(80,60,headless=True)
rows=[]
for w,h,left,visible,loops in ((80,60,0,80,60),(240,160,0,240,22),(240,160,156,40,35),(2048,48,1960,12,20),(8192,48,8100,12,8)):
    img=p.Image(w,h);img.cls(9);dst=p.Image(w,h);ti=p.Tilemap(max(1,w//8),max(1,h//8),img)
    dst.clip(left,0,visible,h)
    for route,call in (
        ('image-affine',lambda:dst.blt(0,0,img,0,0,w,h,None,0.5,1.17)),
        ('map-affine',lambda:dst.bltm(0,0,ti,0,0,w,h,None,0.5,1.17)),
        ('image-perspective',lambda:dst.blt3d(0,0,w,h,img,(w*0.5,h*0.5,24),(4,8,9),60)),
        ('map-perspective',lambda:dst.bltm3d(0,0,w,h,ti,(w*0.5,h*0.5,24),(4,8,9),60))):
        for _ in range(2):call()
        ns=[]
        for _ in range(7):
            t=time.perf_counter_ns()
            for _ in range(loops):call()
            ns.append((time.perf_counter_ns()-t)/loops)
        rows.append(dict(width=w,height=h,left=left,visible=visible,route=route,ns=ns,median_ns=statistics.median(ns)))
print(json.dumps(rows))
