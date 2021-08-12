namespace Metrix.Models
{
    public class MetricInsertPartial
    {
        public string DataPoint { get; set; }
        public string DataGrouping { get; set; }
        public string DataType { get; set; }
        public double DataValueNumeric { get; set; }
    }
}