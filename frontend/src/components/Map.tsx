type MapProps = {
  map: string;
};

export const Map: React.FC<MapProps> = ({ map }) => {
  const rows = map.split("\n");

  return (
    <div>
      {rows.map((_row, index) => {
        const row = _row.split("").map((cell, index) => {
          const color =
            cell === "0" ? "green" : cell === "1" ? "orange" : "rgb(30 58 138)";

          return (
            <span key={index} style={{ color: color }}>
              {cell}
            </span>
          );
        });
        return <div key={index}>{row}</div>;
      })}
    </div>
  );
};

type MapsProps = {
  maps: string[];
};

export const Maps: React.FC<MapsProps> = ({ maps }) => {
  return (
    <div
      className="grid grid-rows-2 grid-flow-col auto-cols-max gap-4"
      style={{ fontFamily: "monospace", fontSize: "0.55rem" }}
    >
      {maps.map((map) => {
        return <Map map={map} />;
      })}
    </div>
  );
};
