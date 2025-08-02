import "./overlay.css";
import { Toolbar, Button } from "@components/index";
import menuIcon from "@assets/icons/menu.svg";
import chevronLeft from "@assets/icons/chevron-left.svg";

interface OverlayProps {
  sidebarButtonHandler: () => void;
  sidebarVisibility: boolean;
}

function Overlay({ sidebarButtonHandler, sidebarVisibility }: OverlayProps) {
  return (
    <div className="overlay">
      <Toolbar />

      <div className="sidebarButtonContainer">
        <Button
          onClick={sidebarButtonHandler}
          icon={sidebarVisibility ? chevronLeft : menuIcon}
        />
      </div>
    </div>
  );
}

export default Overlay;
