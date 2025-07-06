import ReactMap from 'react-map-gl/maplibre';
import 'maplibre-gl/dist/maplibre-gl.css';

function Map() {
  return (
    <ReactMap
      initialViewState={{
        longitude: -122.4,
        latitude: 37.8,
        zoom: 14
      }}
      style={{position: 'fixed', top: 0, left: 0, width: '100%'}}
      mapStyle="https://tiles.openfreemap.org/styles/liberty"
      />
  );
}

export default Map;
