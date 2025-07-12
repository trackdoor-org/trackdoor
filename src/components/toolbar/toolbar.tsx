import "./toolbar.css";
import Button from "../button/button.tsx";

function Toolbar() {
  return (
      <div className="toolbar">
        <Button text="1"/>
        <Button text="2"/>
        <Button text="3"/>
      </div>
  );
}

export default Toolbar;
