using System;

namespace Metrix.Models
{
    public class MetricResult
    {
        public Guid Id { get; set; }
        public string DataPoint { get; set; }
        public string DataGrouping { get; set; }
        public string DataType { get; set; }
        public double DataValueNumeric { get; set; }
        public DateTime CreatedDatetime { get; set; }
    }
}