import { Bar } from "@nivo/bar"

const BarChart = ({ data }) => {
  return (
    <Bar
      data={data}
      height={500}
      width={1200}
      indexBy={"date"}
      keys={["amount"]}
      margin={{ top: 40, right: 150, bottom: 80, left: 50 }}
      valueFormat={v=> `₹: ${v}`}
      colors={{scheme: "set3"}}
      enableLabel={false}
    />
  )
}

export default BarChart
