import { useState, useEffect } from "react";
import { invoke, Channel } from "@tauri-apps/api/core";
import "./App.css";
import {
  Map,
  NavigationControl,
  GeolocateControl,
} from "react-map-gl/maplibre";
import "maplibre-gl/dist/maplibre-gl.css";
import { Overlay, Sidebar } from "@components/index";
import { GpxFile } from "./types/types";

function App() {
  const [gpxFiles, setGpxFiles] = useState<GpxFile[]>([]);
  const [sidebarVisibility, setSidebarVisibility] = useState(true);

  const toggleSidebar = () => {
    setSidebarVisibility(!sidebarVisibility);
  };

  const onEvent = new Channel<GpxFile[]>();
  onEvent.onmessage = (message) => {
    //console.log("Event: get_gpx_files");
    setGpxFiles(message);
  };

  useEffect(() => {
    const getGpxFiles = async () => {
      await invoke("get_gpx_files", { onEvent });
    };

    getGpxFiles();
  }, []);

  return (
    <main>
      <div className="container">
        <Sidebar isCollapsed={!sidebarVisibility} gpxFiles={gpxFiles} />

        <Map
          initialViewState={{
            longitude: -122.4,
            latitude: 37.8,
            zoom: 14,
          }}
          style={{ width: "100%", height: "100%" }}
          mapStyle="https://tiles.openfreemap.org/styles/liberty"
        >
          <NavigationControl position="bottom-right" />
          <GeolocateControl position="bottom-right" />

          <Overlay
            sidebarButtonHandler={toggleSidebar}
            sidebarVisibility={sidebarVisibility}
          />
        </Map>
      </div>
    </main>
  );
}

export default App;
