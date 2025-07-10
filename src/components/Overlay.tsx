import Toolbar from "./Toolbar.tsx"
import Button from "./Button.tsx"
import locationArrow from "../assets/location-arrow.svg"

const overlayStyle = {
  'position': 'absolute',
  'top': '0',
  'left': '0',
  'width': '100%',
  'height': '100%',
  'display': 'flex',
  'flex-direction': 'column',
  'justify-content': 'space-between',
  'z-index': '1',
  'pointer-events': 'none',
};

const LocationButtonStyle = {
  'align-self': 'flex-end',
  'margin': '0px 10px 40px 0px',
};

function Overlay(props) {
  return (
      <div style={overlayStyle}>
        <Toolbar/>

        <div style={LocationButtonStyle}>
          <Button icon={locationArrow}/>
        </div>
      </div>
  );
}

export default Overlay;
