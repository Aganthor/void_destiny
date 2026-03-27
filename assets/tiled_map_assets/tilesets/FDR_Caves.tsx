<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.11.2" name="FDR_Caves" tilewidth="16" tileheight="16" tilecount="1024" columns="32">
 <image source="FDR_Caves.png" trans="ff00ff" width="512" height="512"/>
 <tile id="9" probability="0.5"/>
 <wangsets>
  <wangset name="cave" type="corner" tile="8">
   <wangcolor name="elevation" color="#ff0000" tile="-1" probability="1"/>
   <wangcolor name="floor" color="#00ff00" tile="-1" probability="0.5"/>
   <wangcolor name="outer_walls" color="#0000ff" tile="-1" probability="1"/>
   <wangcolor name="walls" color="#ff7700" tile="-1" probability="1"/>
   <wangcolor name="inner_walls" color="#00e9ff" tile="-1" probability="1"/>
   <wangtile tileid="8" wangid="0,2,0,2,0,2,0,2"/>
   <wangtile tileid="9" wangid="0,2,0,2,0,2,0,2"/>
  </wangset>
 </wangsets>
</tileset>
