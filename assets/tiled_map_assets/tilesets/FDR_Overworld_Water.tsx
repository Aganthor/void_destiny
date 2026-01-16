<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.11.2" name="FDR_Overworld_Water" tilewidth="16" tileheight="16" tilecount="168" columns="12">
 <image source="FDR_Overworld_Water.png" trans="ff00ff" width="192" height="224"/>
 <tile id="0">
  <animation>
   <frame tileid="0" duration="100"/>
   <frame tileid="3" duration="100"/>
   <frame tileid="6" duration="100"/>
   <frame tileid="9" duration="100"/>
   <frame tileid="84" duration="100"/>
   <frame tileid="87" duration="100"/>
   <frame tileid="90" duration="100"/>
   <frame tileid="93" duration="100"/>
  </animation>
 </tile>
 <tile id="1">
  <animation>
   <frame tileid="1" duration="100"/>
   <frame tileid="4" duration="100"/>
   <frame tileid="7" duration="100"/>
   <frame tileid="10" duration="100"/>
   <frame tileid="85" duration="100"/>
   <frame tileid="88" duration="100"/>
   <frame tileid="91" duration="100"/>
   <frame tileid="94" duration="100"/>
  </animation>
 </tile>
 <tile id="2">
  <animation>
   <frame tileid="2" duration="100"/>
   <frame tileid="5" duration="100"/>
   <frame tileid="8" duration="100"/>
   <frame tileid="11" duration="100"/>
   <frame tileid="86" duration="100"/>
   <frame tileid="89" duration="100"/>
   <frame tileid="92" duration="100"/>
   <frame tileid="95" duration="100"/>
  </animation>
 </tile>
 <wangsets>
  <wangset name="Water" type="corner" tile="-1">
   <wangcolor name="Water" color="#ff0000" tile="-1" probability="1"/>
   <wangtile tileid="0" wangid="0,1,0,0,0,1,0,1"/>
   <wangtile tileid="1" wangid="0,1,0,0,0,0,0,1"/>
   <wangtile tileid="2" wangid="0,1,0,1,0,0,0,1"/>
   <wangtile tileid="12" wangid="0,0,0,0,0,1,0,1"/>
   <wangtile tileid="14" wangid="0,1,0,1,0,0,0,0"/>
   <wangtile tileid="24" wangid="0,0,0,1,0,1,0,1"/>
   <wangtile tileid="25" wangid="0,0,0,1,0,1,0,0"/>
   <wangtile tileid="26" wangid="0,1,0,1,0,1,0,0"/>
   <wangtile tileid="36" wangid="0,0,0,1,0,0,0,0"/>
   <wangtile tileid="37" wangid="0,0,0,1,0,1,0,0"/>
   <wangtile tileid="38" wangid="0,0,0,0,0,1,0,0"/>
   <wangtile tileid="48" wangid="0,1,0,1,0,0,0,0"/>
   <wangtile tileid="49" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="50" wangid="0,0,0,0,0,1,0,1"/>
   <wangtile tileid="60" wangid="0,1,0,0,0,0,0,0"/>
   <wangtile tileid="61" wangid="0,1,0,0,0,0,0,1"/>
   <wangtile tileid="62" wangid="0,0,0,0,0,0,0,1"/>
   <wangtile tileid="72" wangid="0,1,0,0,0,1,0,0"/>
   <wangtile tileid="74" wangid="0,0,0,1,0,0,0,1"/>
  </wangset>
 </wangsets>
</tileset>
