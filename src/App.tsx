import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import {
  Map,
  NavigationControl,
  GeolocateControl
} from 'react-map-gl/maplibre';
import 'maplibre-gl/dist/maplibre-gl.css';
import { Overlay, Sidebar } from "@components/index";


function App() {
  const [sidebarVisibility, setSidebarVisibility] = useState(true);
  
   const toggleSidebar = ()=>{
    setSidebarVisibility(!sidebarVisibility);
  }

  return (
    <main>
      <div className="container">
      <Sidebar isCollapsed={!sidebarVisibility}/>

      <Map initialViewState={{
          longitude: -122.4,
          latitude: 37.8,
          zoom: 14
        }}
        style={{'width': '100%', 'height': '100%'}}
        mapStyle="https://tiles.openfreemap.org/styles/liberty"
      >
        <NavigationControl position="bottom-right" />
        <GeolocateControl position="bottom-right" />

        <Overlay sidebarButtonHandler={toggleSidebar} sidebarVisibility={sidebarVisibility}/>
      </Map>
      </div>
    </main>
  );
}

export default App;
