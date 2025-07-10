const style = {
  'position': 'relative',
  'width': '50px',
  'height': '50px',
  'pointer-events': 'auto',
};

const buttonStyle = {
  'width': '100%',
  'height': '100%',
  'fontSize': '20px',
  'background-color': 'white',
  'border-radius': '10px',
  'border-style': 'solid',
  'border-color': 'black',
  'border-width': '1px',
  'pointer-events': 'auto',
};

const iconStyle = {
  'position': 'absolute',
  'top': '25%',
  'left': '25%',
  'width': '50%',
  'height': '50%',
  'pointer-events': 'none',
};

function Button(props) {
  return (
    <div style={style}>
      {props.icon != null ? <img src={props.icon} style={iconStyle}></img> : null}
      <button style={buttonStyle}>{props.text}</button>
    </div>
  );
}

export default Button;
