namespace ProcAPI.Preprocessors
{
    public interface IPreprocessor<T, TR>
    {
        public TR PreprocessData(T data);
    }
}